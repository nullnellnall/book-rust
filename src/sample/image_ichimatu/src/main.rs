fn main() {
    let white = image::Rgb::<u8>([255,255,255]);
    
    let red = image::Rgb::<u8>([255,90,90]);

    let w = 64;

    //drawの中にwhite / red のいずれかが入る。その他はエラー終了
    let draw = |x,y|{
        let (xi,yi)= (x/w, y/w);//整数（i32）なので、64>xは0、128>xは1、192>xは2、…512>xは7。
        /*
            幅512(代入される値は0~511)まである場合、xiのとりうる値は1~8。これを2で割った値（1か0）によって、色を変える。
            例64～127の範囲は常にred。
        */
        /* 
        match (xi % 2 , yi % 2){
            (0,0) => white,
            (1,0) => red,
            (0,1) => red,
            (1,1) => white,
            (_,_) => panic!("error"),
        }
        */
        //if文で表記
        if yi % 2 == 0{
            if xi % 2 == 0 {red} else {white} 
        }else{
            if xi % 2 ==0 {white} else{red}
        }
    };

    let img = image::ImageBuffer::from_fn(512,512,draw);

    img.save("ichimatu.png").unwrap();
}