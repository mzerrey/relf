extern crate serde;
extern crate serde_json;

use std::env;
use std::io::Result;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use warp::Filter;

mod models;
mod frontend;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Default values
    let default_ip = "127.0.0.1";
    let default_port = 5000;
    
    // Parse IP address (first argument)
    let ip_addr = if args.len() > 1 {
        match IpAddr::from_str(&args[1]) {
            Ok(ip) => ip,
            Err(_) => {
                eprintln!("[{}] Invalid IP address format. Using default: {}", 
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), default_ip);
                IpAddr::from_str(default_ip).unwrap()
            }
        }
    } else {
        IpAddr::from_str(default_ip).unwrap()
    };
    
    // Parse port (second argument)
    let port = if args.len() > 2 {
        match args[2].parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("[{}] Invalid port number. Using default: {}", 
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), default_port);
                default_port
            }
        }
    } else {
        default_port
    };
    
    eprintln!("[{}] Starting server on {}:{}", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), ip_addr, port);
    eprintln!("[{}] Server will be accessible from: http://{}:{}/", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), ip_addr, port);
    eprintln!("[{}] Serving static files from dist/ directory", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    // Create socket address from parsed IP and port
    let socket_addr = SocketAddr::new(ip_addr, port);

    // Serve all static files from the dist directory (Trunk output)
    let static_files = warp::fs::dir("dist");
    
    // Serve robots.txt from static directory
    let robots_txt = warp::path("robots.txt")
        .and(warp::get())
        .and(warp::fs::file("static/robots.txt"));
    
    // Redirect root to index.html
    let index_route = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("dist/index.html"));
    
    // SPA fallback - serve index.html for any route not matching a file
    let spa_fallback = warp::any()
        .and(warp::get())
        .and(warp::fs::file("dist/index.html"));

    // Combine all routes
    let routes = robots_txt
        .or(index_route)
        .or(static_files)
        .or(spa_fallback)
        .with(warp::trace::request());

    // Start the server
    warp::serve(routes)
        .run(socket_addr)
        .await;

    Ok(())
}