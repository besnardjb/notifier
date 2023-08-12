use clap::Parser;
use prometheus_http_query::{Error, query};
use serde::Serialize;
use std::error;
use chrono::prelude::*;
use rand::Rng;

async fn query_value(name : &str, server : &str) -> Result<f64, Error>
{
    let response: prometheus_http_query::response::PromqlResult = query(server, name)?.get().await?;
    let r = response.data().as_vector().expect("Success").last().unwrap().sample().value();

    Ok(r)
}
#[derive(Parser)]
struct Args
{
    ///Prometheus Server
    prometheus : String,
    // NotifyD Server
    notifyd : String
}

async fn senf_notify(notif_server: &str, text : &str) -> Result<(), reqwest::Error>
{
    let notif_url = format!("{}/notify", notif_server);
    #[derive(Serialize, Debug)]
    struct NotifyQuery {
        text: String,
    }

    let client = reqwest::Client::new();

    client.post(notif_url).json(&NotifyQuery{
        text : String::from(text)
    }).send().await?;

    Ok(())
}

fn pick_greetings() -> &'static str {
    let possible = [
        "Hello",
        "Bonjour la maison",
        "Ding Dong",
        "Yo la mif",
        "Cher habitants",
        "Comment allez-vous ?",
        "Ça fart ?",
        "Salutations",
        "Belle journée",
        "Quelle joie de vous retrouver",
        "Oyez Oyez",
        "Salut les amis",
        "Bonjour à tous",
        "Coucou tout le monde",
        "Hey là-bas",
        "Salut cher(e) visiteur(euse)",
        "Bonjour chères âmes",
        "Hé ho !",
        "Salutations distinguées",
        "Bien le bonjour",
        "Hello les ami(e)s",
        "Holà !",
        "Bonjour à la joyeuse assemblée",
        "Bonjour et bienvenue",
        "Coucou les copains",
        "Hey, comment ça va ?",
        "Salut les merveilles",
        "Bonjour, cher(e) invité(e)",
        "Haut les cœurs !",
        "Hello tout le monde",
        "Salut, mes chers",
        "Hey, enchanté(e) de vous voir",
        "Bonjour à la troupe",
        "Salut, les ami(e)s proches et lointains",
        "Coucou, mes amis",
        "Salut, les aventuriers",
        "Hey là-dedans !",
        "Bonjour, vous tous",
        "Hé ho, prêts à briller ?",
        "Salut, chers camarades",
        "Hello, êtres fabuleux",
        "Salut, ô joyeux compagnons",
        "Bonjour, étoiles brillantes",
        "Hé vous !",
        "Salut, explorateurs de la vie",
        "Hello, radieux individus",
        "Salut, âmes éclatantes",
        "Hey, chers êtres",
        "Bonjour, ô sources de sourires",
        "Salut, créatures extraordinaires",
        "Hé, vous tous qui égayez ma journée",
        "Salutations et félicitations pour être géniaux",
        "Bonjour, êtres lumineux du monde",
        "Salut, passionnés de l'aventure quotidienne",
        "Hello, lumières éclatantes dans ce monde",
        "Salut, chères étincelles d'humanité",
        "Hé là-bas, créateurs de moments mémorables",
        "Salutations, intrépides voyageurs de la vie",
        "Hello, amis du voyage terrestre",
        "Salut, chers compagnons de l'existence",
        "Hé vous, âmes pétillantes et vibrantes",
        "Salut, amis du monde digital",
        "Hello, explorateurs curieux du quotidien",
        "Salut, artistes de la vie en mouvement",
        "Hé, voyageurs temporels de ce moment précis",
        "Salutations, chers artisans du bonheur"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());    return possible[index]
}

fn pick_report_power() -> &'static str {
    let possible = [
        "Au rapport",
        "A propos de notre production électrique",
        "Coté électricité",
        "Status des Panneaux solaires",
        "Voici les nouvelles",
        "Bulletin de production énergétique",
        "Mise à jour sur l'électricité",
        "Point sur la génération d'énergie",
        "Dernières informations électriques",
        "Bilan énergétique en temps réel",
        "Les chiffres de l'électricité",
        "Actualités énergétiques",
        "Données fraîches sur la production électrique",
        "État actuel de nos sources d'énergie",
        "Performance énergétique récente",
        "Nouvelles du front électrique",
        "Vue d'ensemble de la production d'électricité",
        "Informations sur notre potentiel électrique",
        "Rapport sur la puissance électrique",
        "Analyse de la production énergétique",
        "Mise à jour sur nos ressources électriques",
        "Statistiques énergétiques à jour",
        "Aperçu de la capacité électrique",
        "Éclairage sur la génération d'électricité",
        "Résumé de nos activités électriques",
        "Focus sur la production énergétique",
        "Informations clés sur l'électricité",
        "Vue d'ensemble des sources d'énergie",
        "Instantané de la production électrique",
        "Précisions sur nos performances électriques",
        "État de nos sources d'énergie renouvelable",
        "Exploration de notre potentiel électrique",
        "Zoom sur la génération d'électricité",
        "Détails sur la sortie électrique",
        "Analyse récente de la production énergétique",
        "Vérifications des niveaux électriques",
        "Rapport complet sur l'électricité",
        "État des ressources énergétiques",
        "Aperçu de nos générateurs électriques",
        "Données actuelles sur la production électrique",
        "Bilan énergétique à l'instant T"
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());    return possible[index]
}

fn pick_full() -> &'static str {
    let possible: [&str; 20] = [
        "La batterie a atteint sa capacité maximale de charge.",
        "Stockage énergétique à son apogée : Batterie pleinement chargée !",
        "Préparez-vous à la performance : Batterie à pleine capacité !",
        "Le réservoir est comblé : Batterie à 100% !",
        "Succès énergétique : Batterie complètement chargée !",
        "Le sommet est atteint : Batterie au maximum de sa puissance !",
        "Chargement optimal atteint : Batterie à son niveau maximal !",
        "Capacité énergétique au zénith : Batterie pleine !",
        "Chargeur mis au repos : Batterie est à pleine charge !",
        "Circuit énergétique complet : Batterie est au top !",
        "Prêt pour l'action : Batterie à 100% de sa capacité !",
        "Puissance maximale : Batterie chargée à fond !",
        "Niveau optimal atteint : Batterie est pleine !",
        "Batterie en mode optimal : 100% de charge !",
        "Le plein d'énergie est fait : Batterie à son maximum !",
        "C'est le moment de briller : Batterie au maximum !",
        "Batterie opérationnelle à 100% !",
        "La saturation énergétique est là : Batterie est pleine !",
        "Chargement achevé : Batterie à pleine capacité !",
        "Stockage énergétique complet : Batterie est full !"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index]
}

fn pick_morning_greet() -> &'static str
{
    let possible = [
        "Bonjour ! Que cette nouvelle journée soit remplie de possibilités et de moments merveilleux. Passez une journée lumineuse et positive !",
        "Bonjour à vous ! Que cette journée soit le début d'une aventure passionnante et pleine de succès.",
        "Un nouveau jour se lève, apportant avec lui de nouvelles opportunités. Profitez de chaque instant et brillez de tout votre éclat !",
        "Le soleil se lève sur une nouvelle journée remplie de promesses. Souvenez-vous que chaque jour est une chance de réaliser vos rêves.",
        "Bonjour ! Que votre journée soit aussi radieuse que votre sourire et aussi dynamique que votre esprit.",
        "C'est un nouveau départ, une nouvelle chance de créer, d'apprendre et de grandir. Bonjour et bonne journée à vous !",
        "Que le soleil qui se lève illumine votre journée et éclaire votre chemin vers le succès et le bonheur.",
        "Bonjour ! Puissiez-vous accueillir cette journée avec enthousiasme et positivité. Faites-en une journée exceptionnelle !",
        "Chaque matin est une occasion de recommencer, de laisser derrière soi les regrets d'hier et de construire un avenir meilleur. Bonjour et bonne journée !",
        "Que cette journée vous apporte joie, accomplissements et moments précieux. Bonjour et que votre journée soit fantastique !",
        "Un nouveau lever de soleil signifie de nouvelles possibilités. Laissez votre journée briller aussi intensément que le soleil dans le ciel. Bonjour !",
        "Bonjour ! Que cette journée vous apporte des sourires, de la réussite et tout ce que vous désirez. Profitez pleinement de chaque instant.",
        "Que ce matin soit le début d'une journée exceptionnelle où chaque pas que vous faites vous rapproche de vos objectifs. Bonjour et bonne journée !",
        "Un sourire au réveil est le meilleur moyen de commencer la journée. Alors, souriez et faites de cette journée une merveilleuse aventure ! Bonjour !",
        "Bonjour ! Que cette journée vous offre des opportunités incroyables, des moments de bonheur et des souvenirs inoubliables.",
        "Un nouveau jour, une nouvelle chance de poursuivre vos rêves. Que cette journée soit inspirante et productive. Bonjour et que votre journée soit brillante !",
        "Bonjour ! Puissent vos actions d'aujourd'hui devenir les succès de demain. Profitez de chaque instant de cette journée qui commence.",
        "Chaque nouveau matin est une toile vierge pour peindre votre journée avec des couleurs positives. Bonjour et que votre palette soit riche !",
        "Bonjour ! Vous avez le pouvoir de rendre cette journée incroyable. Embrassez chaque moment avec enthousiasme et gratitude.",
        "Saluez ce matin avec gratitude et détermination. Les opportunités abondent, alors faites-en une journée mémorable !",
        "Que cette journée vous apporte des sourires et des moments heureux à chérir. Bonjour et que votre journée soit spéciale !",
        "Bonjour ! N'oubliez pas que chaque journée est une chance de vous rapprocher de vos rêves. Allez de l'avant avec confiance !",
        "Un nouveau jour est comme un nouveau chapitre de votre vie. Écrivez-le avec passion et courage. Bonjour et que votre histoire soit extraordinaire !",
        "Bonjour ! Que cette journée vous apporte des éclats de rire, des rencontres enrichissantes et des accomplissements à célébrer.",
        "N'oubliez pas de prendre un instant pour vous, de respirer profondément et de commencer cette journée avec calme et sérénité. Bonjour !",
        "Chaque aurore est un cadeau, une chance de créer de merveilleux souvenirs et de vivre des moments précieux. Bonjour et que votre journée soit inoubliable !",
        "Bonjour ! En ce matin frais et nouveau, puissiez-vous ressentir une vague d'énergie positive pour affronter la journée avec force et détermination.",
        "Que ce matin vous apporte la motivation nécessaire pour atteindre vos objectifs et la confiance pour surmonter tout défi qui se présente. Bonjour et bonne journée !",
        "Bonjour ! Soyez reconnaissant(e) pour ce jour qui commence, une opportunité de grandir, d'apprendre et de créer des souvenirs qui dureront toute une vie.",
        "Chaque jour est une page blanche qui attend d'être remplie d'aventures, d'expériences et de moments spéciaux. Bonjour et que votre plume soit inspirée !",
        "Bonjour ! Le monde est prêt à être exploré, découvert et apprécié aujourd'hui. Allez-y avec confiance et curiosité !",
        "Que chaque rayon de soleil qui traverse votre fenêtre vous rappelle que cette journée est remplie de possibilités infinies. Bonjour et que votre journée soit radieuse !",
        "Bonjour ! Puissiez-vous être entouré(e) de positivité, d'amour et de succès tout au long de cette journée qui commence.",
        "Un nouveau matin, un nouveau départ. N'oubliez pas que chaque jour vous offre la chance de faire de nouveaux choix et de créer de nouvelles opportunités. Bonjour !",
        "Bonjour ! Faites de chaque instant de cette journée une pierre précieuse dans le bijou de votre vie. Appréciez chaque étincelle.",
        "Chaque matin est un rappel que la vie est un cadeau précieux. Profitez-en au maximum et partagez votre lumière avec le monde. Bonjour !",
        "Bonjour ! Que votre journée soit remplie de succès, de sourires et de moments de bonheur. Soyez ouvert(e) aux merveilles qui vous entourent.",
        "Que ce matin soit le début d'une série de moments mémorables et d'expériences qui vous nourrissent et vous inspirent. Bonjour et que votre journée soit enrichissante !"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index]
}

fn pick_spend_elec() -> &'static str {
    let possible = [
        "Fais une séance d'entraînement en utilisant un vélo d'appartement électrique.",
        "Lance une soirée de jeux de société avec éclairage de stade.",
        "Fabrique des lumières de Noël personnalisées avec des LEDs.",
        "Organise une compétition de courses de drones lumineux.",
        "Mets en place une projection de film en plein air avec un projecteur.",
        "Crée des expériences de science électrique avec les enfants.",
        "Installe un système de son surround pour une expérience de cinéma à domicile.",
        "Lance un atelier de fabrication de circuits électroniques pour les débutants.",
        "Fais une soirée à thème rétro avec des appareils électriques anciens.",
        "Mets en place un éclairage artistique pour mettre en valeur tes œuvres d'art.",
        "Fais des expériences culinaires avec des appareils électriques variés.",
        "Organise une soirée de danse avec des sols lumineux interactifs.",
        "Fabrique un simulateur de météo à l'intérieur avec des lumières et des sons.",
        "Installe un système de contrôle de la maison intelligente pour optimiser l'énergie.",
        "Crée un spectacle de magie avec des illusions électriques.",
        "Fais des sessions de photographie nocturne avec des sources de lumière variées.",
        "Organise un défilé de mode avec des vêtements et des accessoires lumineux.",
        "Fabrique des gadgets électroniques personnalisés pour ta routine quotidienne.",
        "Mets en place un système de refroidissement d'aquarium sophistiqué.",
        "Crée un jardin vertical éclairé avec des plantes adaptées à la lumière.",
        "Organise une séance de karaoké avec un équipement sonore professionnel.",
        "Fais des soirées d'écoute musicale avec des enceintes haut de gamme.",
        "Installe des fenêtres interactives avec éclairage intégré pour changer la vue.",
        "Crée une installation artistique interactive avec des capteurs électriques.",
        "Mets en place un système de surveillance de la maison pour la sécurité.",
        "Fabrique des accessoires de mode éclairés pour des occasions spéciales.",
        "Organise un atelier de fabrication de jouets électroniques pour les enfants.",
        "Fais des expériences de réalité virtuelle avec des équipements de pointe.",
        "Installe des panneaux solaires pour générer de l'énergie renouvelable.",
        "Crée un spectacle de fontaines lumineuses synchronisées avec la musique.",
        "Organise une journée de hacking éthique pour améliorer la cybersécurité.",
        "Mets en place un spectacle de feux d'artifice synchronisés avec la musique.",
        "Fais une journée de fabrication de gadgets électroniques pour la maison.",
        "Installe un éclairage d'ambiance pour créer différentes atmosphères.",
        "Crée un spectacle de lasers synchronisés avec un système sonore.",
        "Organise une exposition d'art technologique dans ton espace.",
        "Fais des ateliers d'éducation sur l'énergie pour les écoles locales.",
        "Mets en place une mini-serre d'intérieur avec éclairage ajustable.",
        "Crée des vêtements intelligents avec des éléments électroniques intégrés.",
        "Organise des séances de méditation avec des lumières apaisantes.",
        "Fais des démonstrations de drones électriques pour le plaisir.",
        "Mets en place un système de surveillance écologique pour ta région.",
        "Crée un spectacle de lumière et de musique pour les événements locaux.",
        "Organise des ateliers de codage pour apprendre la programmation.",
        "Fais des expériences de peinture lumineuse pour créer des œuvres originales.",
        "Mets en place un système d'illumination nocturne pour ton jardin.",
        "Crée des lampes de bureau personnalisées avec des matériaux recyclés.",
        "Organise une chasse au trésor électronique avec des indices lumineux.",
        "Fais des ateliers de réparation et de recyclage d'appareils électriques.",
        "Mets en place un système d'éclairage interactif pour ta salle de jeux.",
        "Crée des cartes de vœux électroniques avec des LED clignotantes.",
        "T'as pas envie de faire la vaisselle ? On sait tous que les assiettes adorent danser dans le lave-vaisselle !",
        "Et si on lançait le ballon d'eau chaude pour les Jeux Olympiques aquatiques à domicile ?",
        "Oh là là, il y a une machine à laver qui s'ennuie. Elle t'attend pour un tour de manège en mode nettoyage !",
        "N'auriez-vous pas envie de cuisiner un peu avec l'électricité ? Faisons cuire des idées géniales dans le four !",
        "Hé, on dirait que quelqu'un veut jouer avec l'électricité ! Prêt(e) pour une partie de cache-cache dans le noir ?",
        "Pas de cadeaux pour EDF aujourd'hui ! Allons-y, dépensons cette électricité comme si c'était la fête !",
        "Un petit don d'électricité verte pour sauver la planète, ça vous dit ? Appuyez sur le bouton « Éco-Héros » !",
        "Attention, alerte gaspillage d'électricité en cours ! Qui a laissé les photons danser sans musique ?",
        "Hé toi, c'est l'heure de dépenser de l'électricité de manière totalement irresponsable ! Mais chut, c'est un secret !",
        "Fais griller un toast en morse pour communiquer avec les extraterrestres.",
        "Organise une compétition de charge de smartphones, que le meilleur gagne !",
        "Envoie des e-mails à ta machine à laver pour la motiver à travailler plus dur.",
        "Crée une discothèque dans ton salon avec les lumières clignotantes.",
        "Lance un marathon de visionnage de séries, que les pixels s'épuisent !",
        "Fais une bataille d'oreillers avec des coussins chauffants.",
        "Organise une compétition de chargement pour les robots aspirateurs.",
        "Fais des selfies avec toutes les prises électriques de la maison.",
        "Lance une soirée karaoké pour amplifier la voix de la douche.",
        "Construis une réplique de la Tour Eiffel en ampoules.",
        "Crée un festival de danse pour les ampoules clignotantes.",
        "Joue à cache-cache avec le compteur électrique.",
        "Réalise un spectacle de marionnettes éclairé par des lampes de poche.",
        "Organise une course de charge entre ton téléphone et ta tablette."
        ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index]
}

fn pick_dinner() -> &'static str {
    let possible = [
        "Bon appétit pour votre dîner !",
        "C'est l'heure de vous régaler avec un bon repas du soir.",
        "Profitez d'un moment de tranquillité pour le dîner.",
        "Savourez chaque bouchée de votre repas du soir.",
        "Prenez le temps de vous nourrir et de vous détendre.",
        "Que votre dîner soit aussi délicieux que vous le souhaitez.",
        "Un dîner équilibré pour clôturer la journée en beauté.",
        "Prenez plaisir à manger et à vous relaxer ce soir.",
        "Rien de tel qu'un bon repas pour finir la journée en douceur.",
        "Votre dîner est le moment idéal pour vous ressourcer.",
        "Que votre repas du soir apporte satisfaction et réconfort.",
        "Un bon repas pour recharger vos énergies avant la nuit.",
        "Profitez de chaque bouchée de votre dîner ce soir.",
        "Mangez bien et préparez-vous à une soirée reposante.",
        "Nourrissez-vous pour un repos réparateur cette nuit.",
        "Bon appétit ! Que votre dîner soit délicieux et savoureux.",
        "L'heure du dîner : un moment de réconfort bienvenu.",
        "Prenez le temps de manger et de vous détendre ce soir.",
        "Soyez reconnaissant(e) pour ce repas et ce moment de calme.",
        "Profitez de votre repas du soir pour vous relaxer.",
        "Que votre repas soit savoureux et satisfaisant ce soir.",
        "N'oubliez pas de vous nourrir pour bien récupérer la nuit.",
        "Dîner sonne, c'est le moment de vous régaler ce soir.",
        "Faites une pause et appréciez votre repas du soir.",
        "L'heure du dîner est arrivée, profitez-en pour vous détendre.",
        "Savourez chaque instant de votre repas du soir.",
        "Un dîner délicieux pour clore la journée en beauté.",
        "Bon appétit ! Profitez de votre repas du soir.",
        "Laissez-vous emporter par les saveurs de votre dîner.",
        "Prenez le temps de manger et de vous relaxer ce soir.",
        "Que votre repas du soir apporte satisfaction et réconfort.",
        "Nourrissez-vous pour une nuit de sommeil revitalisante.",
        "Profitez de cette parenthèse gustative pour vous détendre.",
        "Un dîner bien mérité pour une nuit de repos réparateur.",
        "C'est l'heure de vous régaler avec un bon repas du soir.",
        "Savourez chaque bouchée et profitez de ce moment de calme.",
        "N'oubliez pas de prendre soin de vous en mangeant équilibré.",
        "Laissez votre dîner préparer votre corps pour la nuit.",
        "Profitez pleinement de votre repas du soir.",
        "Faites en sorte que votre dîner soit une expérience savoureuse.",
        "Que votre dîner vous apporte tranquillité et réconfort.",
        "Nourrissez votre corps et apaisez votre esprit ce soir.",
        "Prenez une pause bien méritée pour savourer votre dîner.",
        "Un dîner qui vous préparera à une nuit de repos réparateur.",
        "C'est le moment de vous régaler ce soir, bon appétit !",
        "Savourez chaque bouchée et profitez de ce temps de détente.",
        "N'oubliez pas de prendre plaisir à manger équilibré.",
        "Laissez-vous emporter par les saveurs de votre dîner.",
        "Profitez de cette pause pour vous ressourcer ce soir.",
        "Faites en sorte que votre dîner soit une expérience délicieuse.",
        "Que votre repas du soir vous apporte satisfaction et réconfort.",
        "Nourrissez-vous pour une nuit de sommeil apaisante.",
        "Prenez une pause bien méritée pour savourer votre repas du soir.",
        "Un dîner qui vous apportera le réconfort nécessaire.",
        "C'est le moment de vous régaler ce soir, bon appétit !",
        "Savourez chaque bouchée et profitez de ce moment de tranquillité.",
        "N'oubliez pas de prendre soin de vous en mangeant équilibré.",
        "Laissez votre dîner apaiser votre corps et votre esprit.",
        "Profitez pleinement de votre repas du soir pour vous détendre.",
        "Faites en sorte que votre dîner soit une expérience satisfaisante.",
        "Que votre repas du soir vous offre un moment de calme.",
        "Nourrissez votre corps et préparez-vous à une nuit de repos.",
        "Prenez une pause bien méritée pour savourer votre dîner ce soir.",
        "Un repas qui vous apportera une nuit de sommeil apaisante.",
        "C'est le moment de vous régaler ce soir, bon appétit !",
        "Savourez chaque bouchée et appréciez ce moment de détente.",
        "N'oubliez pas de prendre plaisir à manger équilibré.",
        "Laissez votre dîner préparer votre esprit et votre corps pour la nuit.",
        "Profitez pleinement de votre repas du soir et reposez-vous bien.",
        "Faites en sorte que votre dîner soit une expérience satisfaisante.",
        "Que votre repas du soir vous apporte une nuit de repos réparateur.",
        "Nourrissez-vous pour une nuit de sommeil paisible.",
        "Prenez une pause bien méritée pour savourer votre repas ce soir.",
        "Un dîner qui vous préparera à une nuit de tranquillité.",
        "C'est le moment de vous régaler ce soir, bon appétit !",
        "Savourez chaque bouchée et reposez-vous bien.",
        "N'oubliez pas de prendre soin de vous en mangeant équilibré.",
        "Laissez votre dîner préparer votre corps et votre esprit pour la nuit.",
        "Profitez pleinement de votre repas du soir pour une nuit apaisante.",
        "Faites en sorte que votre dîner soit une expérience relaxante.",
        "Que votre repas du soir vous offre un sommeil réparateur.",
        "Nourrissez votre corps et détendez-vous pour la nuit.",
        "Prenez une pause bien méritée pour savourer votre repas ce soir.",
        "Un repas qui vous préparera à une nuit de repos revitalisant."
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index]

}

fn pick_lunch() -> &'static str {
    let possible = [
        "Bon appétit pour votre déjeuner !",
        "C'est l'heure de recharger vos batteries avec un bon repas.",
        "Profitez d'une pause bien méritée pour le déjeuner.",
        "Savourez chaque bouchée de votre repas de midi.",
        "N'oubliez pas de vous nourrir et de vous hydrater.",
        "Que votre déjeuner soit aussi délicieux que vous l'espérez.",
        "Un déjeuner équilibré pour une après-midi pleine d'énergie.",
        "Prenez le temps de savourer votre repas de midi.",
        "Rien de tel qu'un bon repas pour reprendre des forces.",
        "Votre pause déjeuner est le moment idéal pour vous détendre.",
        "Faites une pause et dégustez votre repas de midi.",
        "Laissez votre repas vous revigorer pour la suite de la journée.",
        "Profitez de chaque instant de votre pause déjeuner.",
        "Mangez bien et reprenez des forces pour l'après-midi.",
        "Nourrissez-vous pour continuer à briller tout au long de la journée.",
        "Bon appétit ! Que votre repas soit délicieux et satisfaisant.",
        "L'heure du déjeuner : un moment de rechargement bienvenu.",
        "Prenez plaisir à manger et à vous détendre.",
        "Soyez reconnaissant(e) pour ce repas et le temps de repos.",
        "Profitez de cette pause pour vous revitaliser avec un bon repas.",
        "Que votre repas soit savoureux et énergisant.",
        "N'oubliez pas de vous nourrir pour être au top de votre forme.",
        "Midi sonne, c'est le moment de vous régaler.",
        "Faites une pause et appréciez votre déjeuner.",
        "L'heure de la pause déjeuner est arrivée, profitez-en bien.",
        "Savourez chaque instant de votre repas de midi.",
        "Un repas délicieux pour continuer votre journée du bon pied.",
        "Bon appétit ! Régalez-vous avec votre repas de midi.",
        "Laissez-vous emporter par les saveurs de votre déjeuner.",
        "Prenez le temps de manger et de vous relaxer.",
        "Que votre repas de midi soit une source de plaisir et d'énergie.",
        "Nourrissez votre corps et votre esprit pour briller toute la journée.",
        "Profitez de cette parenthèse gustative pour vous détendre.",
        "Un déjeuner bien mérité pour faire le plein d'énergie.",
        "C'est le moment de recharger vos batteries avec un bon repas.",
        "Savourez chaque bouchée et prenez un moment pour vous détendre.",
        "N'oubliez pas de prendre soin de vous en mangeant équilibré.",
        "Laissez votre repas vous préparer à affronter l'après-midi.",
        "Profitez pleinement de votre pause déjeuner.",
        "Faites en sorte que votre repas soit une expérience délicieuse.",
        "Que votre déjeuner vous apporte le réconfort nécessaire.",
        "Nourrissez-vous avec soin pour briller de l'intérieur.",
        "Prenez une pause bien méritée pour savourer votre repas.",
        "Un repas qui vous donnera l'énergie pour continuer la journée.",
        "Laissez le déjeuner raviver vos forces pour la suite.",
        "Profitez de chaque bouchée et de chaque instant de pause.",
        "Mangez avec gratitude et profitez de cette pause.",
        "Que ce repas apporte vitalité et satisfaction à votre journée.",
        "N'oubliez pas de vous hydrater et de prendre soin de vous.",
        "Bon appétit ! Régalez-vous avec un déjeuner délicieux.",
        "Laissez la nourriture nourrir votre esprit et votre corps.",
        "Prenez le temps de vous détendre et de vous ressourcer.",
        "Que votre repas soit un moment de plaisir et de rechargement.",
        "Nourrissez-vous pour briller avec force et enthousiasme.",
        "Profitez de cette pause pour vous revitaliser.",
        "Un repas énergisant pour continuer votre journée en beauté.",
        "C'est l'heure de vous régaler, bon appétit !",
        "Savourez chaque bouchée et savourez ce moment de détente.",
        "N'oubliez pas de prendre plaisir à manger équilibré.",
        "Laissez-vous emporter par les saveurs de votre repas.",
        "Profitez de cette pause déjeuner pour vous détendre.",
        "Faites en sorte que votre déjeuner soit une expérience délicieuse."
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index]
}


fn pick_inject() -> &'static str {
    let possible = [
        "Nous injectons de l'électricité sur le réseau pour illuminer le monde !",
        "EDF vous remercie pour votre contribution en électricité, une énergie plus verte pour tous !",
        "Préparez-vous à briller : Nous partageons notre électricité avec le réseau !",
        "Nos sources d'énergie sont généreuses aujourd'hui : L'électricité excédentaire est envoyée sur le réseau.",
        "L'électricité ne se perd pas, elle se partage ! Nous contribuons au réseau avec notre surplus.",
        "C'est une journée lumineuse : Nous partageons notre énergie avec le réseau électrique !",
        "Un acte généreux : Nous injectons notre électricité dans le réseau pour une communauté plus brillante.",
        "En avant pour une journée électrisante : Notre surplus d'électricité est partagé avec le réseau.",
        "Énergie excédentaire en action : Nous alimentons le réseau avec notre électricité !",
        "La solidarité énergétique est de mise : Nous envoyons notre électricité sur le réseau.",
        "Votre électricité à l'œuvre : Nous partageons notre surplus avec le réseau.",
        "L'énergie circule : Nous contribuons à la stabilité du réseau avec notre électricité.",
        "Branchez-vous sur notre générosité : L'électricité excédentaire est injectée sur le réseau.",
        "Le flux énergétique est en mouvement : Nous partageons notre électricité pour le bien commun.",
        "Un geste lumineux : Nous envoyons notre électricité dans le réseau pour une société plus brillante.",
        "Le réseau s'enrichit : Nous injectons de l'électricité pour éclairer d'autres foyers.",
        "L'excès d'électricité ne se perd pas, il se partage : Nous contribuons au réseau électrique.",
        "En avant vers une meilleure distribution : Nous partageons notre électricité avec le réseau.",
        "Un partage énergétique : Notre électricité excédentaire est mise à profit sur le réseau.",
        "L'énergie circule dans les fils : Nous injectons notre électricité pour le bien de tous.",
        "EDF vous salue pour votre générosité énergétique : Nous alimentons le réseau avec notre surplus.",
        "Notre contribution énergétique : L'excès d'électricité est partagé avec le réseau.",
        "Une connexion lumineuse : Nous envoyons notre électricité dans le réseau pour une journée rayonnante.",
        "Chaque watt compte : Nous partageons notre électricité pour une utilisation optimale.",
        "En avant vers une journée électrique : Nous partageons notre énergie avec le réseau.",
        "L'électricité abonde : Nous alimentons le réseau pour une distribution équilibrée.",
        "Un élan de partage : Notre surplus d'électricité est utile sur le réseau électrique.",
        "Le réseau s'anime : Nous injectons de l'électricité pour une meilleure distribution.",
        "Partage d'énergie : Notre électricité excédentaire est mise à contribution pour tous.",
        "En avant pour une journée brillante : Notre électricité excédentaire est envoyée sur le réseau.",
        "L'excès devient utile : Nous injectons notre électricité pour le bien de la collectivité.",
        "Énergie en mouvement : Nous partageons notre électricité pour une utilisation optimale.",
        "L'électricité circule : Nous contribuons à l'équilibre du réseau avec notre surplus.",
        "Le réseau s'enrichit : Notre énergie est mise à disposition pour un avenir lumineux.",
        "Un geste lumineux : Nous injectons notre électricité pour une distribution équilibrée.",
        "Énergie en partage : Notre surplus d'électricité est utile pour tous les foyers.",
        "En avant vers une distribution équitable : Nous envoyons notre électricité sur le réseau.",
        "L'excédent énergétique ne se perd pas : Nous alimentons le réseau pour un avenir éclairé.",
        "Notre contribution au réseau : L'excès d'électricité est utilisé pour le bien commun.",
        "Une connexion énergétique : Nous partageons notre électricité pour une journée lumineuse.",
        "Chaque kilowatt compte : Nous injectons notre électricité pour une meilleure utilisation.",
        "En avant vers un futur électrisant : Notre électricité excédentaire est mise à profit.",
        "Le réseau s'anime : Nous contribuons à la stabilité du réseau avec notre énergie.",
        "Partage énergétique : Notre électricité est injectée pour le bien de tous.",
        "En avant pour une journée énergétique : Nous partageons notre énergie avec le réseau.",
        "L'électricité se répand : Nous envoyons notre excès d'énergie pour une utilisation optimale.",
        "L'excédent devient précieux : Nous alimentons le réseau pour un avenir éclairé.",
        "Énergie en action : Notre contribution énergétique est mise à disposition du réseau.",
        "Le réseau s'enrichit : Nous injectons notre électricité pour une meilleure distribution.",
        "Un geste énergétique : Nous partageons notre électricité pour un futur lumineux.",
        "Énergie en partage : Notre surplus d'électricité est utilisé pour le bien de la société.",
        "En avant pour une distribution énergétique : Nous envoyons notre énergie sur le réseau."
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index]
}

#[tokio::main]
async fn main()  -> Result<(), Box<dyn error::Error>>
{
    let args = Args::parse();

    let soc = query_value("imeon_battery_soc", args.prometheus.as_str()).await?.round();
    let avgsolar_1h = query_value("avg_over_time(imeon_pv_input_power1[1h])", args.prometheus.as_str()).await?.round();
    let avgpower_1h = query_value("avg_over_time(imeon_em_power[1h])", args.prometheus.as_str()).await?.round();

    let time: DateTime<Local> = Local::now();

    let mut message : String;

    if time.minute() == 0
    {
        message = format!("{} ! Il est {} heure. ", pick_greetings(), time.hour());
    }
    else
    {
        message = format!("{} ! Il est {} heure et {} minutes. ", pick_greetings(), time.hour(), time.minute());
    }

    if time.hour() == 8
    {
        message += pick_morning_greet()
    }else if time.hour()  == 12
    {
        message += pick_lunch();
    }else if time.hour()  == 19
    {
        message += pick_dinner();
    }

    message += format!("{} : ", pick_report_power()).as_str();

    if soc == 100.0
    {
        message += format!("{}. {}. ", pick_full(), pick_spend_elec()).as_str();
    }

    message += format!("Batterie {soc} % ").as_str();
    message += format!("Production moyenne sur la dernière heure {avgsolar_1h} watt heure. ").as_str();
    message += format!("Consommation moyenne sur la dernière heure {avgpower_1h} watt heure. ").as_str();

    if avgpower_1h < 0.0 {
        message += pick_inject();
    }

    //senf_notify(&args.notifyd, &message).await?;

    println!("{}", message);

    Ok(())
}
