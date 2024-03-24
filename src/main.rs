use chrono::{format::format, prelude::*};
use clap::Parser;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use prometheus_http_query::{query, Error};
use rand::Rng;
use serde::Serialize;
use std::error;

async fn query_value(name: &str, server: &str) -> Result<String, Error> {
    let response: prometheus_http_query::response::PromqlResult =
        query(server, name)?.get().await?;
    match response.data().as_vector().expect("Success").last() {
        Some(e) => {
            return Ok(format!("{}", e.sample().value().round()));
        }
        None => {
            return Err(Error::EmptySeriesSelector);
        }
    }
}
#[derive(Parser)]
struct Args {
    /// Prometheus Server
    prometheus: String,
    /// NotifyD Server
    notifyd: String,
    /// Ollama server
    ollama: String,
    /// Ollama model
    model: String,
}

async fn senf_notify(notif_server: &str, text: &str) -> Result<(), reqwest::Error> {
    let notif_url = format!("{}/notify", notif_server);
    #[derive(Serialize, Debug)]
    struct NotifyQuery {
        text: String,
    }

    let client = reqwest::Client::new();

    client
        .post(notif_url)
        .json(&NotifyQuery {
            text: String::from(text),
        })
        .send()
        .await?;

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
        "Hey, enchanté de vous voir",
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
        "Salutations, chers artisans du bonheur",
        "Hello, cher navigateur sur les routes virtuelles",
        "Bonjour, êtres créatifs et innovants",
        "Salut, amis de l'aventure mentale",
        "Hé là-bas, explorateurs du savoir",
        "Salutations, chers penseurs engagés",
        "Hello, chercheurs insatiables de la vérité",
        "Bonjour, êtres d'inspiration et d'imagination",
        "Salut, amis des émotions profondes",
        "Hé vous, créateurs de réalités alternatives",
        "Salutations, chercheurs passionnés du sens de la vie",
        "Hello, êtres généreux et partageurs",
        "Bonjour, compagnons de voyage dans l'univers mental",
        "Salut, amis de la pensée créatrice",
        "Hé là-bas, célébrateurs de la diversité",
        "Salutations, chercheurs éclairés du bien-être",
        "Hello, êtres sensibles et empathiques",
        "Bonjour, compagnons d'esprit dans l'univers numérique",
        "Salut, amis des expériences transcendantales",
        "Hé vous, créateurs de mondes virtuels incroyables",
        "Salutations, chercheurs engagés vers une meilleure humanité",
        "Hello, êtres passionnés d'évolution personnelle",
        "Bonjour, compagnons de voyage dans l'univers des idées",
        "Salut, amis des expériences spirituelles",
        "Hé là-bas, célébrateurs de la créativité et de l'innovation",
        "Salutations, chercheurs du sens de leur existence",
        "Bonjour, êtres passionnés de la découverte",
        "Salut, amis des expériences émotionnelles puissantes",
        "Hé vous, créateurs de mondes inédits et originaux",
        "Salutations, chercheurs engagés vers une meilleure compréhension du monde",
        "Hello, êtres curieux et insatiables d'apprentissage",
        "Bonjour, compagnons de voyage dans l'univers des possibles",
        "Salut, amis des expériences artistiques",
        "Hé là-bas, célébrateurs de la diversité culturelle",
        "Salutations, chercheurs du bonheur et du bien-être",
        "Hello, êtres passionnés de l'amour et de la compassion",
        "Bonjour, compagnons d'esprit dans l'univers des émotions",
        "Salut, amis des expériences spirituelles et métaphysiques",
        "Hé vous, créateurs de mondes virtuels inédits et innovants",
        "Salutations, chercheurs engagés vers une meilleure harmonie avec la nature",
        "Hello, êtres conscients et éclairés de leur impact sur le monde",
        "Bonjour, compagnons de voyage dans l'univers des possibles infinis",
        "Salut, amis des expériences d'épanouissement personnel",
        "Hé là-bas, célébrateurs de la liberté et de l'indépendance",
        "Salutations, chercheurs passionnés de la vérité absolue",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    possible[index]
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
        "Bilan énergétique à l'instant T",
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
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
        "Stockage énergétique complet : Batterie est full !",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn pick_morning_greet() -> &'static str {
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
    return possible[index];
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
    return possible[index];
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
        "Un repas qui vous préparera à une nuit de repos revitalisant.",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
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
        "Faites en sorte que votre déjeuner soit une expérience délicieuse.",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
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
    return possible[index];
}

fn pick_break() -> &'static str {
    let possible = [
        "C'est l'heure de faire une pause gourmande ! Profitez de votre goûter pour recharger vos batteries.",
        "Prenez une pause bien méritée et savourez votre goûter. Un petit plaisir pour vous redonner de l'énergie.",
        "Quoi de mieux qu'un délicieux goûter pour se remonter le moral ? Prenez une pause et régalez-vous !",
        "Que ce goûter vous apporte une douceur bienvenue dans votre journée. Prenez le temps de vous détendre et de savourer.",
        "Un petit creux ? L'heure du goûter est arrivée. Dégustez quelque chose de délicieux et profitez-en !",
        "Offrez-vous une pause sucrée et délicieuse avec un goûter qui fait plaisir aux papilles.",
        "Prenez une pause et comblez votre faim avec un goûter délicieux. Votre journée en sera d'autant plus agréable.",
        "C'est l'instant parfait pour une pause gourmande. Laissez-vous tenter par un délicieux goûter et appréciez chaque bouchée.",
        "Le goûter est le moment idéal pour recharger vos batteries et vous préparer pour la suite de la journée. Profitez-en !",
        "Faites une pause et accordez-vous un moment de plaisir avec un goûter savoureux. Vous l'avez bien mérité !",
        "N'oubliez pas de vous octroyer une pause pour reprendre des forces avec un délicieux goûter. Bon appétit !",
        "Le goûter, c'est l'occasion de faire une pause et de prendre soin de vous en dégustant quelque chose de délicieux.",
        "Profitez de ce moment de détente pour savourer un goûter qui éveille vos papilles et vous fait sourire.",
        "C'est l'heure de ravir vos sens avec un goûter délectable. Prenez une pause et profitez de chaque instant.",
        "Laissez-vous tenter par un goûter délicieux et faites une pause bien méritée. Votre journée n'en sera que plus agréable !",
        "Un petit en-cas pour vous redonner de l'énergie et vous faire plaisir. Prenez le temps de déguster votre goûter.",
        "N'oubliez pas de vous accorder une pause gourmande pour vous régaler avec un goûter savoureux. Bon appétit !",
        "Que ce goûter soit un moment de douceur et de réconfort dans votre journée. Prenez le temps de vous détendre.",
        "Savourez chaque bouchée de votre goûter et profitez de cette parenthèse gourmande pour vous ressourcer.",
        "Prenez une pause et savourez votre goûter avec délectation. Un petit plaisir qui illumine votre journée !",
        "Le goûter est le moment idéal pour prendre une pause, vous détendre et déguster quelque chose de délicieux.",
        "Faites une pause et offrez-vous un moment de plaisir avec un goûter qui mettra un sourire sur votre visage.",
        "Le goûter, c'est la pause rêvée pour combler votre faim et vous offrir une touche de bonheur sucré.",
        "Profitez de ce moment pour vous faire plaisir avec un goûter savoureux. Vous le méritez bien !",
        "N'oubliez pas de vous offrir une pause gourmande pour savourer un délicieux goûter. Bon appétit !",
        "Que votre goûter soit un instant de réconfort et de plaisir. Prenez le temps de déguster chaque bouchée.",
        "Un goûter délicieux pour vous redonner de l'énergie et ajouter une touche de douceur à votre journée.",
        "Prenez une pause et savourez chaque instant de votre goûter. C'est un petit plaisir qui fait toute la différence.",
        "Le goûter, c'est le moment de se faire plaisir avec une petite gourmandise. Profitez-en pour vous détendre.",
        "Faites une pause et régalez-vous avec un goûter délicieux. Vous avez bien travaillé, vous le méritez !",
        "Le goûter, c'est le moment de prendre une pause, de savourer et de vous chouchouter. Bon appétit !",
        "Profitez de ce moment pour faire une pause et apprécier votre goûter. Un petit plaisir qui égaye votre journée.",
        "N'oubliez pas de vous accorder un instant de répit avec un goûter savoureux. C'est le moment de vous faire plaisir !"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn pick_goodnight() -> &'static str {
    let possible = [
        "Bonne soirée ! Que votre nuit soit douce et reposante, remplie de rêves merveilleux.",
        "Il est temps de se détendre et de profiter d'une belle soirée. Passez une nuit paisible et réparatrice.",
        "Que votre soirée soit paisible et votre nuit remplie de doux rêves. Bonne nuit et à demain !",
        "Profitez de chaque moment de votre soirée, puis laissez-vous emporter dans un sommeil profond et réparateur. Bonne nuit !",
        "Bonne soirée à vous ! Puissiez-vous trouver le calme et la sérénité pendant la nuit qui approche.",
        "Que la magie de la soirée enveloppe votre cœur de douceur et que votre nuit soit remplie de rêves enchanteurs.",
        "Il est temps de laisser derrière vous les soucis de la journée. Passez une agréable soirée et une nuit paisible.",
        "Bonne soirée ! Puissiez-vous vous reposer confortablement et vous réveiller rafraîchi(e) et prêt(e) pour un nouveau jour.",
        "Que votre soirée soit aussi belle que les étoiles dans le ciel et que votre nuit soit calme et réparatrice.",
        "Profitez de la tranquillité de la soirée et laissez vos soucis s'apaiser pendant votre sommeil. Bonne nuit !",
        "Bonne soirée ! Profitez de ce temps pour vous relaxer et vous préparer pour une nuit de repos bien méritée.",
        "Que votre soirée soit douce comme une mélodie apaisante et que votre nuit soit paisible et reposante.",
        "Il est temps de fermer les yeux et de laisser la magie de la nuit vous emporter. Bonne soirée et bonne nuit !",
        "Que votre soirée soit remplie de moments agréables et que votre nuit soit un havre de paix et de tranquillité.",
        "Bonne soirée ! Puissent vos soucis s'évaporer au coucher du soleil et vos rêves vous emmener vers des mondes enchantés.",
        "Que votre soirée soit un doux prélude à une nuit de sommeil réparateur. Profitez pleinement de ces moments de repos.",
        "Il est temps de clore cette journée en beauté. Passez une soirée agréable et une nuit paisible. Bonne nuit !",
        "Bonne soirée à vous ! Que votre nuit soit un doux voyage au pays des rêves et de la détente.",
        "Que chaque étoile dans le ciel illumine votre soirée et apporte une lueur de tranquillité à votre nuit.",
        "Bonne soirée ! Que votre nuit soit remplie de rêves tendres et de repos bien mérité.",
        "Il est temps de se laisser aller au sommeil et de profiter d'une nuit régénérante. Bonne soirée et bonne nuit !",
        "Que votre soirée soit belle et que votre nuit soit remplie de quiétude et de repos. Bonne nuit à vous !",
        "Bonne soirée ! Puissiez-vous trouver le confort et la paix dans la nuit qui s'annonce.",
        "Profitez de chaque instant de votre soirée et laissez-vous porter par un sommeil apaisant. Bonne nuit !",
        "Bonne soirée à vous ! Que votre nuit soit enveloppée de douceur et de rêves apaisants.",
        "Que la soirée vous offre un moment de détente bien mérité et que la nuit vous apporte un repos réparateur.",
        "Bonne soirée ! Laissez vos préoccupations de côté et préparez-vous pour une nuit de repos bienfaisant.",
        "Profitez de cette soirée pour vous ressourcer et recharger vos énergies. Passez une nuit sereine et reposante.",
        "Bonne soirée ! Que votre nuit soit tissée de rêves enchanteurs et de douceur infinie.",
        "Que la soirée vous apporte le réconfort et la tranquillité dont vous avez besoin. Bonne nuit et à demain !",
        "Bonne soirée à vous ! Puissiez-vous trouver le calme intérieur nécessaire pour une nuit de sommeil paisible.",
        "Que votre soirée soit un moment de relaxation et que votre nuit soit un doux voyage vers la quiétude.",
        "Bonne soirée ! Que vos rêves soient doux et votre sommeil réparateur pour un lendemain radieux.",
        "Profitez de cette soirée pour vous détendre et vous reposer. Que votre nuit soit paisible et revitalisante.",
        "Bonne soirée à vous ! Puissent les étoiles veiller sur votre sommeil et vous offrir des rêves magiques.",
        "Que votre soirée soit baignée de sérénité et que votre nuit soit un refuge de tranquillité. Bonne nuit !"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn pick_motiv_morning() -> &'static str {
    let possible = [
        "C'est le moment de faire une pause et de vous accorder un moment de réflexion. Prenez une profonde respiration et recentrez-vous.",
        "Prenez quelques instants pour vous détendre et réfléchir à vos objectifs matinaux. Restez concentré(e) et déterminé(e) pour le reste de la matinée.",
        "Faites une pause dans vos activités matinales et prenez un moment pour réfléchir à votre journée. Que votre réflexion vous guide vers la productivité et le succès.",
        "Le milieu de la matinée est un bon moment pour réfléchir à ce que vous avez accompli jusqu'à présent et pour ajuster vos priorités pour le reste de la journée.",
        "Profitez de cette pause matinale pour faire le point sur vos tâches et vos objectifs. Restez positif(ve) et motivé(e) pour ce qui vient !",
        "Prenez un instant pour apprécier le progrès que vous avez déjà réalisé ce matin. Utilisez cette réflexion pour booster votre motivation et votre énergie.",
        "Le milieu de la matinée est l'occasion parfaite pour vous reconnecter avec vos objectifs et renouveler votre engagement envers eux. Continuez à avancer avec détermination.",
        "Accordez-vous un court moment pour vous rappeler pourquoi vous vous êtes levé(e) ce matin. Utilisez cette réflexion pour guider vos actions pour le reste de la journée.",
        "C'est le moment de faire une pause café mentale. Profitez de cette occasion pour réfléchir à vos réussites et pour planifier vos prochaines étapes.",
        "Prenez quelques instants pour respirer profondément et vous féliciter pour vos efforts matinaux. Votre réflexion vous aidera à maintenir un état d'esprit positif.",
        "Utilisez ce moment de réflexion pour recharger vos énergies et pour vous rappeler que chaque action que vous entreprenez vous rapproche de vos objectifs.",
        "Profitez de cette pause pour évaluer vos priorités et pour vous assurer que vos activités matinales sont alignées avec vos aspirations à long terme.",
        "Prenez un instant pour vous remercier pour votre engagement envers vos objectifs. Votre dévouement pave la voie vers le succès que vous méritez.",
        "Le milieu de la matinée est le moment idéal pour renforcer votre motivation. Rappellez-vous pourquoi vos objectifs sont importants et inspirez-vous pour continuer.",
        "Faites une pause et imaginez le sentiment de satisfaction que vous ressentirez en accomplissant vos tâches matinales avec excellence. Cela vaut chaque effort !",
        "Profitez de ce moment pour exprimer de la gratitude envers vous-même pour chaque petit pas en avant que vous avez fait jusqu'à présent. Continuez à grandir !",
        "Prenez quelques instants pour visualiser votre réussite et pour ressentir la fierté qui accompagne l'atteinte de vos objectifs. Vous êtes sur la bonne voie !",
        "Le milieu de la matinée est le moment idéal pour rappeler à votre esprit que chaque effort que vous investissez dans vos objectifs vous rapproche du succès.",
        "Faites une pause et célébrez les petites victoires que vous avez déjà remportées ce matin. Chaque pas compte et vous rapproche de votre destination.",
        "Utilisez ce moment pour vous encourager et vous rappeler que chaque étape, aussi petite soit-elle, vous propulse dans la direction de vos rêves.",
        "Prenez un instant pour réaliser que chaque moment que vous consacrez à vos objectifs construit la base solide de votre réussite future. Continuez à bâtir !",
        "Le milieu de la matinée est un rappel que vos efforts matinaux sont les fondations sur lesquelles vous construisez votre journée fructueuse. Restez concentré(e) !"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn pick_motiv_afternoon() -> &'static str {
    let possible = [
        "Faites une pause dans vos activités de l'après-midi et prenez un moment pour vous recentrer. Respirez profondément et laissez vos pensées s'apaiser.",
        "C'est le moment idéal pour une pause de réflexion en milieu d'après-midi. Prenez un moment pour recharger vos énergies et reprendre le reste de la journée avec vigueur.",
        "Profitez de cette pause pour réfléchir à vos réalisations de l'après-midi et pour planifier vos prochaines étapes. Restez concentré(e) et motivé(e) !",
        "Le milieu de l'après-midi est un bon moment pour faire le point sur votre progression et pour réajuster vos objectifs si nécessaire. Continuez à avancer avec détermination.",
        "Prenez quelques instants pour méditer sur vos tâches et vos accomplissements de l'après-midi. Restez positif(ve) et concentré(e) pour le reste de la journée.",
        "Faites une pause et réfléchissez à la façon dont vous pouvez maximiser votre efficacité pendant le reste de l'après-midi. Restez concentré(e) sur vos objectifs.",
        "C'est le moment de faire un petit bilan de vos réussites et de vous donner un encouragement pour le reste de l'après-midi. Vous avez déjà accompli beaucoup !",
        "Prenez un instant pour évaluer votre niveau d'énergie et ajustez votre rythme pour optimiser votre performance pendant le reste de la journée.",
        "Le milieu de l'après-midi est le moment parfait pour vous donner une dose d'inspiration. Prenez une pause pour lire quelque chose de motivant ou pour visualiser vos réussites.",
        "Faites une pause et rappelez-vous pourquoi vous avez entrepris les tâches de l'après-midi. Utilisez cette réflexion pour maintenir votre concentration et votre détermination.",
        "Profitez de cette pause pour faire un pas en arrière et apprécier le chemin que vous avez parcouru aujourd'hui. Vous êtes sur la bonne voie vers vos objectifs.",
        "C'est le moment de reprendre votre souffle et de réfléchir à vos accomplissements de l'après-midi. Chaque petit pas vous rapproche davantage de votre réussite.",
        "Prenez quelques instants pour réaliser à quel point vous avez été productif(ve) jusqu'à présent. Continuez sur cette lancée positive pour le reste de la journée.",
        "Le milieu de l'après-midi est l'occasion de vous rappeler que chaque effort que vous investissez porte ses fruits. Restez engagé(e) et persévérant(e) !",
        "Faites une pause et réfléchissez à la manière dont vous pouvez optimiser vos prochaines heures. Utilisez cette réflexion pour rester concentré(e) sur vos objectifs.",
        "Utilisez ce moment pour réaffirmer votre détermination à atteindre vos objectifs. Les défis peuvent surgir, mais vous êtes prêt(e) à les surmonter avec succès.",
        "C'est le moment idéal pour reconnaître votre travail acharné et pour vous rappeler que chaque effort contribue à votre progression. Continuez à avancer avec confiance.",
        "Prenez quelques instants pour vous rappeler que chaque moment d'effort compte, peu importe sa taille. Chaque pas en avant vous rapproche de la réussite.",
        "Le milieu de l'après-midi est le moment propice pour reconnaître votre ténacité et pour apprécier vos réalisations. Continuez à poursuivre vos objectifs avec enthousiasme.",
        "Faites une pause et réfléchissez à la façon dont vous pouvez maintenir votre élan pour le reste de la journée. Votre engagement vous mène vers le succès.",
        "Utilisez ce moment pour visualiser vos objectifs déjà accomplis et pour vous encourager à persévérer. Vous avez le pouvoir de réaliser ce que vous désirez.",
        "C'est le moment de recharger vos énergies pour le dernier tronçon de la journée. Réfléchissez à vos réalisations pour rester motivé(e) jusqu'à la fin.",
        "Prenez quelques instants pour ressentir la fierté de ce que vous avez accompli jusqu'à présent aujourd'hui. Vous êtes sur la bonne voie vers le succès total.",
        "Le milieu de l'après-midi est l'occasion de renouveler votre engagement envers vos objectifs. Chaque instant que vous consacrez vous rapproche de vos rêves.",
        "Faites une pause et rappelez-vous que vous êtes capable de surmonter les défis qui se présentent. Votre persévérance est la clé de votre réussite.",
        "Utilisez ce moment pour vous encourager et pour maintenir votre détermination. Vous avez déjà accompli beaucoup et le reste de la journée est prometteur.",
        "C'est le moment idéal pour faire une pause et célébrer vos progrès. Votre persévérance vous a mené(e) à ce point, et il y a encore tellement à réaliser !"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn now_weather() -> &'static str {
    let possible = [
        "Météo actuelle :",
        "Conditions météorologiques en ce moment :",
        "Voici le temps actuel :",
        "Regardez le ciel :",
        "La météo du moment :",
        "Présentement à l'extérieur :",
        "Observations météo actuelles :",
        "Le temps ici et maintenant :",
        "Les dernières nouvelles météo :",
        "Instantané météo :",
        "Un coup d'œil à la météo actuelle :",
        "Actualité météo :",
        "L'état du temps en ce moment :",
        "Point sur la météo actuelle :",
        "Instantané climatique :",
        "État de l'atmosphère :",
        "Métrologie en direct :",
        "Vue d'ensemble météo :",
        "Météorologie locale :",
        "Observations atmosphériques :",
        "Conditions climatiques :",
        "Situation actuelle des nuages :",
        "Informations sur les précipitations :",
        "Température, humidité et vents :",
        "Prévision courte terme :",
        "Météo de la région :",
        "Les conditions présentes en temps réel :",
        "Rapport météorologique :",
        "Vue synoptique du temps :",
        "État du climat à ce moment-là :",
        "Météo du moment :",
        "Conditions climatiques actuelles :",
        "Températures, précipitations et vents :",
        "Les conditions météorologiques en ce moment :",
        "Présentation des conditions météorologiques :",
        "Météo de l'endroit présent :",
        "Conditions actuelles climatiques :",
        "Informations sur les températures et les vents :",
        "Vue d'ensemble météorologique :",
        "Météo en temps réel :",
        "Conditions actuelles de l'air :",
        "Température, humidité et pression :",
        "État du climat présent :",
        "Prévisions météorologiques courtes durées :",
        "Observations atmosphériques en direct :",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn strong_wind() -> &'static str {
    let possible = [
        "Attention, vents forts en cours !",
        "Préparez-vous à des rafales de vent importantes !",
        "Vents puissants signalés, tenez-vous prêt(e) !",
        "Soyez vigilant(e), les vents sont très forts !",
        "Vigilance accrue : vents forts enregistrés.",
        "Gare aux bourrasques, vents intenses en action !",
        "Les vents soufflent fort, restez à l'abri.",
        "Vents vigoureux en vue, prenez vos précautions !",
        "Vent violent signalé, soyez prudent(e) !",
        "Vent puissant enregistré, restez en sécurité !",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn cold_temp() -> &'static str {
    let possible = [
        "Risque de gel, veillez à prendre des précautions.",
        "Températures basses à prévoir, restez au chaud.",
        "Attention, risque de gel nocturne !",
        "Précaution hivernale : restez au chaud.",
        "Gare au froid ! Habillez-vous chaudement.",
        "Vigilance météo : risque de gel.",
        "Températures en chute, préparez-vous.",
        "Alerte gel : prenez vos précautions.",
        "Protégez-vous du gel nocturne, restez au chaud.",
        "Nuit froide en perspective, préparez-vous.",
        "Températures négatives à l'horizon, couvrez-vous bien !",
        "Le mercure descend : préparez-vous à affronter le froid.",
        "Gel annoncé : gardez vos vêtements chauds à portée de main.",
        "Le froid arrive, restez au chaud et confortable.",
        "Météo frisquette en vue, restez bien emmitouflé(e).",
        "Soyez prêt(e) pour des températures glaciales.",
        "Attention, gel attendu. Sortez bien couvert(e) !",
        "Froid intense à prévoir, gardez-vous au chaud.",
        "Des frissons en perspective, préparez-vous à grelotter.",
        "Nuit glaciale à l'horizon, adoptez des mesures pour vous réchauffer.",
        "Le froid fait son apparition, prenez soin de vous.",
        "Les températures chutent : restez confortablement au chaud.",
        "Risque de gel : pensez à protéger vos tuyaux du froid.",
        "Gel attendu, pensez à couvrir vos plantes et à vous préparer.",
        "Froid intense en approche, soyez prêt(e) à affronter les frimas.",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn high_temp() -> &'static str {
    let possible = [
        "Attention, températures élevées en cours !",
        "Préparez-vous à la chaleur intense !",
        "Températures élevées signalées, restez au frais !",
        "Soyez vigilant(e), la chaleur est intense !",
        "Vigilance accrue : températures élevées enregistrées.",
        "Gare à la canicule, restez hydraté(e).",
        "Les températures grimpent, trouvez un endroit frais.",
        "Températures brûlantes en vue, prenez vos précautions !",
        "Chaleur étouffante signalée, soyez prudent(e) !",
        "Chaleur intense enregistrée, protégez-vous du soleil !",
        "Attention, risque de coup de chaleur ! Restez au frais.",
        "Gardez-vous au frais : températures élevées signalées.",
        "Chaleur accablante en approche, restez hydraté(e).",
        "Températures en hausse, prenez soin de vous.",
        "Alerte chaleur : restez à l'ombre et buvez de l'eau.",
        "Canicule annoncée : protégez-vous du soleil.",
        "Des records de chaleur enregistrés, adoptez des mesures de précaution.",
        "Le thermomètre s'emballe : restez à l'abri de la chaleur.",
        "Risque de coup de soleil élevé, utilisez de la protection solaire.",
        "Attention aux insolations : hydratez-vous régulièrement.",
        "Températures torrides en perspective, trouvez un endroit frais.",
        "Restez à l'ombre : chaleur extrême annoncée.",
        "Précaution canicule : adoptez des mesures de refroidissement.",
        "Chaleur intense attendue, prenez des précautions contre la déshydratation.",
        "Alerte canicule : protégez-vous et restez à l'abri de la chaleur.",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    return possible[index];
}

fn pick_notif_day() -> &'static str {
    let possible = [
        "Nous sommes aujourd'hui dans",
        "La date merveilleuse de",
        "Voici le jour où",
        "Le moment historique de",
        "Dans ce jour précieux,",
        "Nous sommes au rendez-vous de",
        "C'est le jour de",
        "Le jour qui marque l'anniversaire de",
        "En ce jour tranquille,",
        "Le temps de célébrer est arrivé",
        "Le beau jour de",
        "Nous sommes aujourd'hui les heureux",
        "Le jour de",
        "Voici le jour que nous avions tous attendu",
        "C'est un jour magnifique",
        "Nous sommes aujourd'hui pour fêter",
        "Le beau jour de",
        "Voici le jour que nous avions tous espérés",
        "Le jour est venu pour nous rêver",
        "Nous sommes aujourd'hui pour redémarrer",
        "Le jour est venu de triompher",
        "Dans ce jour, nous avons la chance d'être",
        "Nous sommes aujourd'hui pour réaliser nos rêves",
        "Le jour est venu de réunir notre famille",
        "Dans ce jour merveilleux, nous vivons pleinement",
        "Nous sommes aujourd'hui pour créer des souvenirs",
        "Le jour est venu de faire le meilleur de nous-mêmes",
        "Voici le jour qui a changé tout",
        "Nous sommes aujourd'hui pour célébrer la vie",
        "Le jour est venu de nous apprendre une leçon",
        "Dans ce jour, nous avons l'opportunité de réaliser nos objectifs",
        "Nous sommes aujourd'hui pour exprimer notre amour",
        "Le jour est venu de nous rappeler la beauté du monde",
        "Dans ce jour, nous vivons chaque instant",
        "Nous sommes aujourd'hui pour changer notre vie",
        "Le jour est venu de nous donner espérance",
        "Dans ce jour merveilleux, rien ne nous semble impossible",
        "Nous sommes aujourd'hui pour réaliser nos rêves les plus fous",
        "Le jour est venu de nous apprendre à être heureux",
        "Dans ce jour, nous sommes tous unis",
        "Nous sommes aujourd'hui pour redécouvrir notre passion",
        "Le jour est venu de nous donner une nouvelle chance",
        "Dans ce jour, tout est possible",
        "Nous sommes aujourd'hui pour réaliser nos ambitions",
        "Le jour est venu de nous donner un nouveau départ",
        "Dans ce jour, nous vivons pleinement chaque instant",
        "Nous sommes aujourd'hui pour célébrer la vie et l'amour",
        "Le jour est venu de nous apprendre à être heureux ensemble",
        "Dans ce jour, rien ne peut nous arrêter",
        "Nous sommes aujourd'hui pour réaliser nos objectifs les plus élevés",
        "Le jour est venu de nous rappeler que la vie est une aventure",
        "Dans ce jour, nous sommes tous des héros",
        "Nous sommes aujourd'hui pour faire le monde un meilleur lieu",
        "Le jour est venu de nous rappeler que la vie est courte",
        "Dans ce jour, nous vivons chaque instant avec passion",
        "Nous sommes aujourd'hui pour célébrer nos succès et nos défaites",
        "Le jour est venu de nous rappeler que la famille est tout important",
        "Dans ce jour, nous sommes tous des créateurs",
        "Le jour est venu de nous rappeler que chaque jour est unique",
        "Dans ce jour, nous vivons chaque instant avec amour",
        "Nous sommes aujourd'hui pour célébrer notre humanité",
        "Le jour est venu de nous rappeler que nous sommes tous des êtres fragiles",
        "Dans ce jour, nous vivons chaque instant avec gratitude",
        "Nous sommes aujourd'hui pour réaliser notre plein potentiel",
        "Le jour est venu de nous rappeler que la vie est un don",
        "Dans ce jour, nous vivons chaque instant avec passion et amour",
        "Nous sommes aujourd'hui pour célébrer notre existence",
        "Le jour est venu de nous rappeler que la vie est une aventure merveilleuse",
        "Dans ce jour, nous sommes tous des explorateurs",
        "Nous sommes aujourd'hui pour réaliser notre rêve le plus cher",
        "Le jour est venu de nous rappeler que la vie est courte et précieuse",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et gratitude",
        "Nous sommes aujourd'hui pour réaliser nos objectifs les plus audacieux",
        "Le jour est venu de nous rappeler que chaque action compte",
        "Dans ce jour, nous sommes tous des créateurs exceptionnels",
        "Nous sommes aujourd'hui pour réaliser notre vision du monde",
        "Le jour est venu de nous rappeler que la vie est une aventure incroyablement belle",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et espoir",
        "Nous sommes aujourd'hui pour réaliser nos cauchemars les plus terribles",
        "Le jour est venu de nous rappeler que chaque nouvelle journée est une opportunité",
        "Dans ce jour, nous sommes tous des innovateurs",
        "Nous sommes aujourd'hui pour réaliser notre vision de la société idéale",
        "Le jour est venu de nous rappeler que chaque instant peut être une aventure",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et résonance",
        "Nous sommes aujourd'hui pour réaliser nos ambitions les plus élevées",
        "Le jour est venu de nous rappeler que chaque nouvelle expérience est une occasion de grandir",
        "Dans ce jour, nous sommes tous des explorateurs passionnés",
        "Nous sommes aujourd'hui pour réaliser notre rêve de paix et d'harmonie dans le monde",
        "Le jour est venu de nous rappeler que chaque instant peut être un moment clé",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et intensité",
        "Nous sommes aujourd'hui pour réaliser nos objectifs les plus audacieux et les plus incroyables",
        "Le jour est venu de nous rappeler que chaque action peut avoir des répercussions immenses",
        "Dans ce jour, nous sommes tous des visionnaires",
        "Nous sommes aujourd'hui pour réaliser notre rêve de changement et de transformation",
        "Le jour est venu de nous rappeler que chaque nouvelle journée est une opportunité inestimable",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et dévotion",
        "Nous sommes aujourd'hui pour réaliser nos ambitions les plus chères et les plus ambitieuses",
        "Le jour est venu de nous rappeler que chaque nouvelle expérience peut nous apprendre quelque chose nouveau",
        "Dans ce jour, nous sommes tous des inventeurs",
        "Nous sommes aujourd'hui pour réaliser notre rêve de paix et d'unité dans le monde",
        "Le jour est venu de nous rappeler que chaque instant peut être une aventure magique",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et curiosité",
        "Nous sommes aujourd'hui pour réaliser nos objectifs les plus ambiteux et les plus incroyables",
        "Le jour est venu de nous rappeler que chaque nouvelle rencontre peut changer notre vie",
        "Dans ce jour, nous sommes tous des explorateurs intrépides",
        "Nous sommes aujourd'hui pour réaliser notre rêve de changement et d'évolution",
        "Le jour est venu de nous rappeler que chaque nouvelle expérience peut nous apprendre quelque chose invaluable",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et inspiration",
        "Nous sommes aujourd'hui pour réaliser nos ambitions les plus audacieuses et les plus incroyables",
        "Le jour est venu de nous rappeler que chaque nouvelle aventure peut nous apprendre quelque chose nouveau",
        "Dans ce jour, nous sommes tous des créateurs visionnaires",
        "Nous sommes aujourd'hui pour réaliser notre rêve de paix et d'harmonie universelle",
        "Le jour est venu de nous rappeler que chaque nouvelle expérience peut nous faire grandir",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et résonance profonde",
        "Nous sommes aujourd'hui pour réaliser nos objectifs les plus ambiteux et les plus inattendus",
        "Le jour est venu de nous rappeler que chaque nouvelle rencontre peut être une aventure magique",
        "Dans ce jour, nous sommes tous des inventeurs visionnaires",
        "Nous sommes aujourd'hui pour réaliser notre rêve de changement et de transformation profonde",
        "Le jour est venu de nous rappeler que chaque nouvelle expérience peut nous apprendre quelque chose inestimable",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et enthousiasme",
        "Nous sommes aujourd'hui pour réaliser nos ambitions les plus chères et les plus ambitieuses",
        "Le jour est venu de nous rappeler que chaque nouvelle aventure peut être une aventure de découverte",
        "Dans ce jour, nous sommes tous des explorateurs passionnés",
        "Nous sommes aujourd'hui pour réaliser notre rêve de paix et d'unité dans le monde entier",
        "Le jour est venu de nous rappeler que chaque nouvelle rencontre peut nous apprendre quelque chose nouveau et important",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et curiosité insatiable",
        "Nous sommes aujourd'hui pour réaliser nos objectifs les plus ambitieux et les plus incroyables",
        "Le jour est venu de nous rappeler que chaque nouvelle expérience peut nous apprendre quelque chose invaluable et profondément enrichissant",
        "Dans ce jour, nous vivons chaque instant avec passion, amour et enthousiasme intense",
        "Nous sommes aujourd'hui pour réaliser notre rêve de changement et d'évolution profonde et durable",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible.len());
    possible[index]
}

async fn safe_query(value: &str, server: &str) -> String {
    match query_value(value, server).await {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            "".to_string()
        }
    }
}

struct WeatherCode {
    code: u32,
    description: &'static str,
}

fn get_weather_code(code: u32) -> &'static str {
    let weather_codes = [
        WeatherCode {
            code: 113,
            description: "Le temps est dégagé et ensoleillé",
        },
        WeatherCode {
            code: 116,
            description: "Le temps est partiellement nuageux",
        },
        WeatherCode {
            code: 119,
            description: "Le ciel est nuageux",
        },
        WeatherCode {
            code: 122,
            description: "Le ciel est couvert",
        },
        WeatherCode {
            code: 143,
            description: "Il y a de la brume",
        },
        WeatherCode {
            code: 176,
            description: "Des pluies éparses sont à proximité",
        },
        WeatherCode {
            code: 179,
            description: "Il y a des chutes de neige éparses à proximité",
        },
        WeatherCode {
            code: 182,
            description: "Il y a du verglas épars à proximité",
        },
        WeatherCode {
            code: 185,
            description: "Il y a de la bruine verglaçante épars à proximité",
        },
        WeatherCode {
            code: 200,
            description: "Des orages sont épars à proximité",
        },
        WeatherCode {
            code: 227,
            description: "Des chasse-neige sont en action",
        },
        WeatherCode {
            code: 230,
            description: "Un blizzard est en cours",
        },
        WeatherCode {
            code: 248,
            description: "Il y a du brouillard",
        },
        WeatherCode {
            code: 260,
            description: "Il y a du brouillard givrant",
        },
        WeatherCode {
            code: 263,
            description: "Il y a des averses de bruine légère",
        },
        WeatherCode {
            code: 266,
            description: "Il y a de la bruine légère",
        },
        WeatherCode {
            code: 281,
            description: "Il y a de la bruine verglaçante légère",
        },
        WeatherCode {
            code: 284,
            description: "Il y a de la bruine verglaçante forte",
        },
        WeatherCode {
            code: 293,
            description: "Il y a des averses de pluie légère",
        },
        WeatherCode {
            code: 296,
            description: "Il y a de la pluie légère",
        },
        WeatherCode {
            code: 299,
            description: "Par moments, il y a une pluie modérée",
        },
        WeatherCode {
            code: 302,
            description: "Il y a de la pluie modérée",
        },
        WeatherCode {
            code: 305,
            description: "Par moments, il y a une pluie forte",
        },
        WeatherCode {
            code: 308,
            description: "Il y a de fortes pluies",
        },
        WeatherCode {
            code: 311,
            description: "Il y a de la pluie verglaçante légère",
        },
        WeatherCode {
            code: 314,
            description: "Il y a de la pluie verglaçante modérée ou forte",
        },
        WeatherCode {
            code: 317,
            description: "Il y a de la neige légère",
        },
        WeatherCode {
            code: 320,
            description: "Il y a de la neige modérée ou forte",
        },
        WeatherCode {
            code: 323,
            description: "Il y a des averses de neige légère éparses",
        },
        WeatherCode {
            code: 326,
            description: "Il y a de la neige légère",
        },
        WeatherCode {
            code: 329,
            description: "Il y a des averses de neige modérée éparses",
        },
        WeatherCode {
            code: 332,
            description: "Il y a de la neige modérée",
        },
        WeatherCode {
            code: 335,
            description: "Il y a des averses de neige forte éparses",
        },
        WeatherCode {
            code: 338,
            description: "Il y a de fortes chutes de neige",
        },
        WeatherCode {
            code: 350,
            description: "Il y a des grêlons",
        },
        WeatherCode {
            code: 353,
            description: "Il y a des averses de pluie légère",
        },
        WeatherCode {
            code: 356,
            description: "Il y a des averses de pluie modérée ou forte",
        },
        WeatherCode {
            code: 359,
            description: "Il y a des averses de pluie torrentielles",
        },
        WeatherCode {
            code: 362,
            description: "Il y a des averses de grésil légères",
        },
        WeatherCode {
            code: 365,
            description: "Il y a des averses de grésil modéré ou fort",
        },
        WeatherCode {
            code: 368,
            description: "Il y a des averses de neige légères",
        },
        WeatherCode {
            code: 371,
            description: "Il y a des averses de neige modérées ou fortes",
        },
        WeatherCode {
            code: 374,
            description: "Il y a des averses légères de grêlons",
        },
        WeatherCode {
            code: 377,
            description: "Il y a des averses modérées ou fortes de grêlons",
        },
        WeatherCode {
            code: 386,
            description: "Il y a de la pluie légère dans la région avec des éclairs",
        },
        WeatherCode {
            code: 389,
            description: "Il y a de la pluie modérée ou forte dans la région avec des éclairs",
        },
        WeatherCode {
            code: 392,
            description: "Il y a de la neige légère dans la région avec des éclairs",
        },
        WeatherCode {
            code: 395,
            description: "Il y a de la neige modérée ou forte dans la région avec des éclairs",
        },
    ];

    for weather_code in weather_codes.iter() {
        if code == weather_code.code {
            return weather_code.description;
        }
    }
    "Code météo inconnu"
}

async fn weather_message(prometheus_url: &str) -> String {
    let temperature_celsius: String = safe_query(
        "last_over_time(temperature_celsius{forecast=\"current\"}[2h])",
        prometheus_url,
    )
    .await;
    let temperature_celsius_max: String = safe_query(
        "last_over_time(temperature_celsius_maximum{forecast=\"0d\"}[2h])",
        prometheus_url,
    )
    .await;
    let temperature_celsius_min: String = safe_query(
        "last_over_time(temperature_celsius_minimum{forecast=\"0d\"}[2h])",
        prometheus_url,
    )
    .await;
    let wind_speed_kmph: String = safe_query(
        "last_over_time(windspeed_kmph{forecast=\"current\"}[1h])",
        prometheus_url,
    )
    .await;
    let humidity_percentage: String = safe_query(
        "last_over_time(humidity_percentage{forecast=\"current\"}[2h])",
        prometheus_url,
    )
    .await;
    let cloudover_percentage: String = safe_query(
        "last_over_time(cloudcover_percentage{forecast=\"current\"}[2h])",
        prometheus_url,
    )
    .await;
    let weather_code: String = safe_query(
        "last_over_time(weather_code{forecast=\"current\"}[2h])",
        prometheus_url,
    )
    .await;

    if temperature_celsius == "" {
        // No data
        return "".to_string();
    }

    // Traiter les données pour générer le message météo
    let mut message = String::new();

    let temperature_value: f32 = temperature_celsius.parse().unwrap_or(0.0);
    let temperature_value_max: f32 = temperature_celsius_max.parse().unwrap_or(0.0);
    let temperature_value_min: f32 = temperature_celsius_min.parse().unwrap_or(0.0);

    let wind_speed_value: f32 = wind_speed_kmph.parse().unwrap_or(0.0);
    let humidity_value: f32 = humidity_percentage.parse().unwrap_or(0.0);
    let weather_code_value: u32 = weather_code.parse().unwrap_or(0);
    let cloudover_percentage_value: u32 = cloudover_percentage.parse().unwrap_or(0);

    // Générer le message en fonction des valeurs obtenues
    message.push_str(now_weather());
    message.push_str(&format!(" {}.", get_weather_code(weather_code_value)));

    if cloudover_percentage_value > 0 {
        message.push_str(&format!(" Couverture nuageuse {}%.", cloudover_percentage));
    }

    message.push_str(&format!(
        " Température actuelle : {} degrés, Minimale {}, Maximale {}. ",
        temperature_value, temperature_value_min, temperature_value_max
    ));
    message.push_str(&format!(
        "Vitesse du vent : {} km par heure, ",
        wind_speed_value
    ));
    message.push_str(&format!("Humidité : {}%. ", humidity_value));

    if wind_speed_value > 40.0 {
        message.push_str(strong_wind());
    }
    if temperature_value_min < 5.0 {
        message.push_str(cold_temp());
    } else if temperature_value_max > 27.0 {
        message.push_str(high_temp());
    }

    message
}

async fn electricity_message(prometheus_url: &str) -> String {
    let mut message = String::new();

    let soc = safe_query("imeon_battery_soc", prometheus_url).await;
    let avgsolar_1h = safe_query("avg_over_time(imeon_pv_input_power1[1h])", prometheus_url).await;
    let avgpower_1h = safe_query("avg_over_time(imeon_em_power[1h])", prometheus_url).await;

    message += format!(" {} : ", pick_report_power()).as_str();

    if soc == "100.0" {
        message += format!("{}. {}. ", pick_full(), pick_spend_elec()).as_str();
    }

    message.push_str(&format!("Batterie {soc} % "));
    message.push_str(&format!(
        "Production moyenne sur la dernière heure {avgsolar_1h} watt heure. "
    ));
    message.push_str(&format!(
        "Consommation moyenne sur la dernière heure {avgpower_1h} watt heure. "
    ));

    if let Ok(v) = avgpower_1h.parse::<f32>() {
        if v < 0.0 {
            message += pick_inject();
        }
    }

    message
}

fn to_fr_day(day: Weekday) -> &'static str {
    match day {
        Weekday::Mon => "Lundi",
        Weekday::Tue => "Mardi",
        Weekday::Wed => "Mercredi",
        Weekday::Thu => "Jeudi",
        Weekday::Fri => "Vendredi",
        Weekday::Sat => "Samedi",
        Weekday::Sun => "Dimanche",
    }
}

fn to_fr_month(m: u32) -> &'static str {
    match m as i32 {
        1 => "Janvier",
        2 => "Février",
        3 => "Mars",
        4 => "Avril",
        5 => "Mai",
        6 => "Juin",
        7 => "Juillet",
        8 => "Août",
        9 => "Septembre",
        10 => "Octobre",
        11 => "Novembre",
        12 => "Décembre",
        _ => "Erreur",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let time: DateTime<Local> = Local::now();

    let mut message: String = "".to_string();

    let dayow = to_fr_day(time.weekday());
    let day = time.day();
    let month = to_fr_month(time.month());

    message.push_str(&format!("{} !", pick_greetings()));

    message.push_str(&format!(" {} {dayow} {day} {month}. ", pick_notif_day()));

    if time.minute() == 0 {
        message.push_str(&format!(" Il est {} heure. ", time.hour()));
    } else {
        message.push_str(&format!(
            " Il est {} heure et {} minutes. ",
            time.hour(),
            time.minute()
        ));
    }

    if time.hour() == 8 {
        message += pick_morning_greet();
    } else if time.hour() == 10 {
        message += pick_motiv_morning();
    } else if time.hour() == 12 {
        message += pick_lunch();
    } else if time.hour() == 15 {
        message += pick_motiv_afternoon();
    } else if time.hour() == 16 {
        message += pick_break();
    } else if time.hour() == 19 {
        message += pick_dinner();
    } else if time.hour() == 20 {
        message += pick_goodnight();
    }

    message += " ";

    message += weather_message(args.prometheus.as_str()).await.as_str();
    message += electricity_message(args.prometheus.as_str()).await.as_str();

    let ollama = Ollama::new(args.ollama, 443);

    let res = ollama
        .generate(GenerationRequest::new(
            args.model.to_string(),
            message.to_string(),
        ))
        .await;

    match res {
        Ok(res) => {
            println!("Did run Ollama model {} on the message", args.model);
            message = res.response;
        }
        Err(e) => println!("Error : {}", e),
    }

    println!("{}", message);

    senf_notify(&args.notifyd, &message).await?;

    Ok(())
}
