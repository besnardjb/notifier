use clap::Parser;
use prometheus_http_query::{Error, query};
use serde::Serialize;
use std::error;
use chrono::prelude::*;


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


#[tokio::main(flavor = "current_thread")]
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
        message = format!("Ding Dong ! Il est {} heure",time.hour());
    }
    else
    {
        message = format!("Ding Dong ! Il est {} heure et {} minutes",time.hour(), time.minute());
    }

    message += "Rapport de status des panneaux solaires. ";

    if soc == 100.0
    {
        message += "Attention la batterie des paneaux solaires est pleine. Il est temps de dépenser plein de courant de manière irresponsable. "
    }

    message += format!("Batterie {soc} % ").as_str();
    message += format!("Production moyenne sur la dernière heure {avgsolar_1h} watt heure. ").as_str();
    message += format!("Consommation moyenne sur la dernière heure {avgpower_1h} watt heure. ").as_str();

    if avgpower_1h < 0.0 {
        message += "Injection d'électricité sur le réseau. "
    }

    senf_notify(&args.notifyd, &message).await?;

    println!("{}", message);

    Ok(())
}
