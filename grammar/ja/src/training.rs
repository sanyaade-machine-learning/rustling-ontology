use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "零");
    example!(v, check_integer(1), "一");
    example!(v, check_integer(2), "二");
    example!(v, check_integer(3), "三");
    example!(v, check_integer(4), "四");
    example!(v, check_integer(5), "五");
    example!(v, check_integer(6), "六");
    example!(v, check_integer(7), "七");
    example!(v, check_integer(8), "八");
    example!(v, check_integer(9), "九");
    example!(v, check_integer(10), "十");
    example!(v, check_integer(20), "二十");
    example!(v, check_integer(30), "三十");
    example!(v, check_integer(40), "四十");
    example!(v, check_integer(90), "九十");
    example!(v, check_integer(33), "三十三");
    example!(v, check_integer(96), "九十六");
    example!(v, check_integer(14), "十四");
    example!(v, check_integer(11), "十一");
    example!(v, check_ordinal(7), "七番目");
    example!(v, check_ordinal(11), "十一番目");
    example!(v, check_ordinal(91), "九十一番目");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(5.0, Some("degree")), "五度");
    example!(v, check_temperature(5.0, Some("celsius")), "摂氏五度");
    example!(v, check_temperature(5.0, Some("fahrenheit")), "華氏五度");
    example!(v, check_temperature(25.0, Some("degree")), "二十五度");
    example!(v, check_temperature(-10.0, Some("degree")), "マイナス十度");
    example!(v, check_temperature(-10.0, Some("degree")), "零下十度");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 0, 0, 1]), "一秒間");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1]), "一分間");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "一時間");
    example!(v, check_duration!([0, 0, 0, 5]), "五日間");
    example!(v, check_duration!([0, 10]), "十ヶ月間", "十カ月間");
}

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 10]), "一昨日", "二千十三年二月十日", "前の日曜日", "先週の日曜日");
    example!(v, check_moment!(c, [2013, 2, 11]), "昨日", "前の日", "前日");
    example!(v, check_moment!(c, [2013, 2, 13]), "明日", "次の日", "二千十三年二月十三日", "今週の水曜日", "バレンタインデーの前の日"); 
    example!(v, check_moment!(c, [2013, 2, 20]), "次の水曜日");
    example!(v, check_moment!(c, [2013, 2, 14]), "二千十三年二月十四日", "バレンタインデーの日", "今週の木曜日");
    example!(v, check_moment!(c, [2013, 2, 14]), "次の木曜日");
    example!(v, check_moment!(c, [2013, 2, 15]), "二千十三年二月十五日", "今週の金曜日");
    example!(v, check_moment!(c, [2013, 2, 15]), "次の金曜日");
    example!(v, check_moment!(c, [2013, 2, 16]), "今週の土曜日", "二千十三年二月十六日", "二月十六日");
    example!(v, check_moment!(c, [2013, 2, 16]), "次の土曜日");
    example!(v, check_moment!(c, [2013, 2, 17]), "二千十三年二月十七日", "二月十七日", "今週の日曜日");
    example!(v, check_moment!(c, [2013, 2, 17]), "次の日曜日");
    example!(v, check_moment!(c, [2013, 2, 18]), "二千十三年二月十八日", "二月十八日", "来週の月曜日");
    example!(v, check_moment!(c, [2013, 2, 18]), "次の月曜日");
    example!(v, check_moment!(c, [2013, 2, 19]), "二千十三年二月十九日", "二月十九日", "一週間後", "次の火曜日", "来週の火曜日");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "午後三時十五分", "十五時十五分");
    example!(v, check_moment!(c, [2013, 2, 12, 13, 30]), "十三時三十分", "十三時半", "午後一時半", "午後一時三十分");
    example!(v, check_moment!(c, [2014, 1, 1]), "元旦", "元日", "二千十四年一月一日");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "現在", "今", "今すぐ", "今すぐに", "只今", "ただいま");
    example!(v, check_moment!(c, [2013, 12, 23]), "二千十三年十二月二十三日", "十二月二十三日", "天皇誕生日", "クリスマスイブの前の日", "クリスマスイブの前日");
    example!(v, check_moment!(c, [2013, 2, 12, 14, 15]), "十四時十五分", "午後二時十五分");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 45]), "午前五時四十五分");
    example!(v, check_moment_span!(c, [2013, 2, 16, 00], [2013, 2, 18, 00]), "週末", "今週末");
    example!(v, check_moment!(c, [2012]), "去年", "昨年", "前年", "前の年");
    example!(v, check_moment!(c, [2013]), "今年", "当年");
    example!(v, check_moment!(c, [2014]), "来年", "新年");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 12]), "朝", "午前", "今朝");
    example!(v, check_moment!(c, [2013, 12, 25]), "クリスマス", "次のクリスマス",  "二千十三年十二月二十五日", "十二月二十五日");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 3]), "昨晚");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 3]), "今晚", "今夜");
    example!(v, check_moment!(c, [2013, 8, 1]), "二千十三年八月一日", "八月一日");
    example!(v, check_moment!(c, [2013, 6, 1]), "二千十三年六月一日", "六月一日");
    example!(v, check_moment!(c, [2013, 8, 11]), "山の日");
    example!(v, check_moment!(c, [2017, 8, 22]), "二千十七年八月二十二日");
    example!(v, check_moment!(c, [2013, 2, 12]), "今日", "当日");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "月曜日の朝", "次の月曜日の朝", "来週の月曜日の朝", "月曜日の午前中", "次の月曜日の午前中", "来週の月曜日の午前中");
    example!(v, check_moment!(c, [2013, 10, 7]), "二千十三年十月七日");
    example!(v, check_moment!(c, [2013, 2, 5]), "二千十三年二月五日", "一週間前", "前の火曜日", "先週の火曜日");
    example!(v, check_moment!(c, [2013, 3, 1]), "二千十三年三月一日");
    example!(v, check_moment!(c, [2015, 3, 3]), "二千十五年三月三日");
    example!(v, check_moment!(c, [2013, 2, 15]), "二千十三年二月十五日" , "バレンタインデーの次の日");
    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "二千十三年二月十五日四時から十二時まで", "三日後の朝");
    example!(v, check_moment!(c, [2013, 2, 19]), "二千十三年二月十九日");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "今週");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "先週", "前の週");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "来週", "次の週");
    example!(v, check_moment!(c, [2013, 1]), "二千十三年一月", "先月", "前の月");
    example!(v, check_moment!(c, [2013, 3]), "二千十三年三月", "来月", "次の月");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 0]), "過去二秒間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 1], [2013, 2, 12, 4, 30, 4]),  "次の三秒間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "過去二分間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "次の三分間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 8]), "早朝", "明け方");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "過去二日間");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "明日から三日間", "次の三日間");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "過去二週間");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "次の月曜日から三週間", "次の三週間");
    example!(v, check_moment_span!(c, [2013, 12, 1], [2014, 3, 1]), "今年の十二月から来年の二月まで");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 7, 1]), "今年の三月から六月まで", "三月から六月まで" );
    example!(v, check_moment_span!(c, [2011], [2013]), "過去二年間");
    example!(v, check_moment_span!(c, [2014], [2017]), "次の三年間");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "二千十三年二月十二日午後三時", "今日の十五時");
    example!(v, check_moment!(c, [2015, 4, 14]), "二千十五年四月十四日");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "今晚八時" , "今夜八時", "今日二十時", "今日の午後八時");
    example!(v, check_moment!(c, [2013, 3, 8]), "女性の日");
    example!(v, check_moment!(c, [2013, 2, 15]), "三日後");
    example!(v, check_moment!(c, [2013, 2, 9]), "三日前");

    example!(v, check_moment_span!(c, [2013, 8, 1], [2013, 8, 11]), "八月の上旬");
    example!(v, check_moment_span!(c, [2013, 10, 1], [2013, 10, 11]), "十月の上旬");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 3, 4]), "来月の始め");
    example!(v, check_moment_span!(c, [2013, 1, 10], [2013, 1, 21]), "先月の半ば");
    example!(v, check_moment_span!(c, [2013, 1, 1], [2013, 1, 11]), "先月の上旬");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 3, 4]), "来月の始め");
    example!(v, check_moment_span!(c, [2013, 3, 10], [2013, 3, 21]), "来月の半ば");
    example!(v, check_moment_span!(c, [2013, 8, 29], [2013, 9, 1]), "八月末");
    example!(v, check_moment_span!(c, [2013, 1, 29], [2013, 2, 1]), "先月末");
    example!(v, check_moment_span!(c, [2013, 3, 29], [2013, 4, 1]), "来月末");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "午後八時", "夜の八時");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "午後三時");
    example!(v, check_moment!(c, [2013, 2, 13, 10]), "明日の朝十時");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "昼の三時");
    example!(v, check_moment!(c, [2013, 2, 13, 10]), "明朝十時");
    example!(v, check_moment!(c,  [2013, 2, 12, 17]), "午後五時");
    example!(v, check_moment_span!(c, [2013, 2, 15], [2013, 2, 21]), "十五日から二十日まで");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 3, 6]), "一日から五日", "一日から五日まで");
    example!(v, check_moment_span!(c, [2013, 2, 28], [2013, 3, 4]), "二十八日から三日", "二十八日から三日まで");
    example!(v, check_moment!(c, [2013, 2, 10, 14, 57]), "一昨日の午後三時三分前");
}