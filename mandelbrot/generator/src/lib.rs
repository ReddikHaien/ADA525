
use anyhow::Result;
use http::{Method, StatusCode};
use serde::Deserialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Deserialize)]
struct MandlebrotQueryDto{
    pub scale: f64,
    pub offset_x: i32,
    pub offset_y: i32,
    pub pixel_x: i32,
    pub pixel_y: i32,
    pub img_w: i32,
    pub img_h: i32,
    pub slice_w: i32,
    pub slice_h: i32,
}



/// A simple Spin HTTP component.
#[http_component]
fn handle_generator(req: Request) -> Result<Response> {

    match req.method(){
        &Method::POST => {
            if let Some(body) = req.body(){
                match serde_json::from_slice::<MandlebrotQueryDto>(&body){
                    Ok(data) => {
                        let img = compute_mandlebrot(data);
                        Ok(http::Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/octet-stream")
                            .body(Some(img.into()))?)
                    },
                    Err(e) => {
                        println!("failed to parse json {}",e);
                        Ok(http::Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Some(e.to_string().into()))?)
                    }
                }
            }
            else{
                println!("missing body");
                Ok(http::Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Some("Missing body".into()))?)
            }
        },
        _ => Ok(http::Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(None)?)
    }
}

fn compute_mandlebrot(query: MandlebrotQueryDto)-> Vec<u8> {
    let min_x = -2.00 * query.scale;
    let max_x =  0.47 * query.scale;
    let min_y = -1.12 * query.scale;
    let max_y = 1.12 * query.scale;

    let mut out = vec![0u16;(query.slice_w*query.slice_h) as usize];

    let mut out_index = 0;
    for py in (query.pixel_y)..(query.pixel_y + query.slice_h){
        for px in (query.pixel_x)..(query.pixel_x + query.slice_w){
            let fitted_x = ((px + query.offset_x) as f64) / (query.img_w as f64);
            let fitted_y = ((py + query.offset_y) as f64) / (query.img_h as f64);
            let x0 = (fitted_x)*(max_x - min_x) + min_x;
            let y0 = (fitted_y)*(max_y - min_y) + min_y;
            let mut x = 0f64;
            let mut y = 0f64;
            let mut i = 0u16;

            while (x.powi(2) + y.powi(2) <= 4.0) & (i < 1000) {
                let tmp = x.powi(2) - y.powi(2) + x0;
                y = 2.0*x*y + y0;
                x = tmp;
                i+=1;
            }

            out[out_index] = i.to_le();
            out_index+=1;
        }
    }
    
    let out = out.into_iter().flat_map(|x| [((x >> 8) & 0xff) as u8, (x & 0xff) as u8]).collect::<Vec<_>>();
    out
}