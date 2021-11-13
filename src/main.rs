use std::borrow::Borrow;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main()
{
    let mut num: Vec<i32> = vec![0; 10];
    let xml_path = Path::new("/run/media/zlfn/temp/enwiki.xml");
    let xml_file = File::open(xml_path).unwrap();
    let file = std::io::BufReader::new(xml_file);

    let mut i = 0;
    for result in parse_mediawiki_dump::parse(file)
    {
        let page = result.unwrap();
        let mut text = page.text;
        let re = Regex::new(r"shell\|1=").unwrap(); //리다이렉트 문서 제거
        let mut text = re.replace_all(text.as_str(), "");
        let re = Regex::new(r"[^0-9 ]").unwrap();  //숫자랑 공백만 남기기
        let mut text = re.replace_all(text.as_ref(), " ");
        let re = Regex::new(r"\s[1-9][0-9][0-9][0-9]\s").unwrap();  //(년도일 가능성이 높은) 네자리 숫자 싹다 날리기
        let mut text = re.replace_all(text.as_ref(), " ");
        let re = Regex::new(r"0").unwrap();  //0 날리기
        let mut text = re.replace_all(text.as_ref(), " ");
        let re = Regex::new(r"[^/s]\d").unwrap(); //숫자 앞자리만 남기기
        let mut text = re.replace_all(text.as_ref(), " ");
        let re = Regex::new(r"\s").unwrap(); //공백 제거
        let mut text = re.replace_all(text.as_ref(), "");

        if (!(text.is_empty()))
        {
            for c in text.chars()
            {
                match c
                {
                    '1' => { num[1] = num[1] + 1; }
                    '2' => { num[2] = num[2] + 1; }
                    '3' => { num[3] = num[3] + 1; }
                    '4' => { num[4] = num[4] + 1; }
                    '5' => { num[5] = num[5] + 1; }
                    '6' => { num[6] = num[6] + 1; }
                    '7' => { num[7] = num[7] + 1; }
                    '8' => { num[8] = num[8] + 1; }
                    '9' => { num[9] = num[9] + 1; }
                    _ => { println!("Error"); }
                }
            }
        }

        if(i%1000 == 0) {
            println!("{} pages",i);
            println ! ("1: {}", num[1]);
            println ! ("2: {}", num[2]);
            println ! ("3: {}", num[3]);
            println ! ("4: {}", num[4]);
            println ! ("5: {}", num[5]);
            println ! ("6: {}", num[6]);
            println ! ("7: {}", num[7]);
            println ! ("8: {}", num[8]);
            println !("9: {}", num[9]);
            println!("");
        }
        i = i+1;
    }
}

