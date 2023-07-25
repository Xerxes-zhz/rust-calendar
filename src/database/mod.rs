use tiberius::{AuthMethod, Client, Config,Row};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use futures::StreamExt;

#[tokio::main]
pub async fn link() -> anyhow::Result<Vec<String>> {
    let mut config = Config::new();

    config.host("192.168.1.10");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("zhanghaozhe", "09be3m#%"));
    config.database("WIND");
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // To be able to use Tokio's tcp, we're using the `compat_write` from
    // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
    // traits from the `futures` crate.
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    let stream = client
        .query(
            "
            Select 
                TRADE_DAYS AS TradingDay
            From WIND.db_datareader.ASHARECALENDAR
            WHERE S_INFO_EXCHMARKET='SSE'
            Order By TradingDay
            ",
            &[],
        )
        .await?;
    let mut ans:Vec<String> = Vec::new();
    let mut rows = stream.into_row_stream();
    while let Some(row) = rows.next().await{
        match row {
            Ok(row)=>{
                let item = row.get::<&str,_>("TradingDay").unwrap_or_default().to_string();
                ans.push(item);
            }
            Err(e)=>{
                eprintln!("error: {}",e);
            }
            
        }
    }
    Ok(ans)
}
