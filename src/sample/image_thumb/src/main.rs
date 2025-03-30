use image::{self,imageops,GenericImageView};

/*
画像をサムネに変換するツール　らしい。
*/
fn main() {
    let size = 128;//px?
    let args:Vec<String> = std::env::args()/*Arg型の取得。Iterator実装してるのでfor出来る */.collect()/*Vec型に変換 */;

    if args.len() < 2 { //引数2個以上ないとエラー扱い。
        println!("[USAGE] image_thumb imagefile");
        return;
    }

    let infile = String::from(&args[1]);
    let outfile = format!("{}-thumb.png",infile);

    println!("input:{}",infile);
    println!("output:{}",outfile);
    
    //画像読み込み
    let mut img = image::open(infile).expect("画像ファイルが読みだせません");

    //get demention (寸法) tuple.(width , height) 
    let dim = img.dimensions();

    //tuple.0⇒一番目の要素を指定。
    let w = if dim.0 > dim.1 {
        dim.1
    }else{
        dim.0
    };
    
    let mut img2 = imageops::crop(//可変ビューの取得
        &mut img, //加工対象の画像
        (dim.0-w)/2,//画像切り取りの開始地点（x座標）※なるべく真ん中から切り抜く
        (dim.1-w)/2,//画像切り取りの開始地点（y座標）※なるべく真ん中から切り抜く
        w,//画像切り取りサイズ（x座標）
        w//画像切り取りサイズ（y座標）
    ).to_image();

    let image3 = imageops::resize(&mut img2,size,size,imageops::Lanczos3);

    image3.save(outfile).unwrap();
}