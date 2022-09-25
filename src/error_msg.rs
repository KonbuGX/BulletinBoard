use phf::phf_map;

//エラーメッセージを定義
static ERROR_MSG: phf::Map<&'static str, &str> = phf_map! {
    "EM_0001" => "{}が未入力です。",
    "EM_ACCT_0001" => "アカウント名が重複しています。",
    "EM_ACCT_0002" => "パスワードの文字数を8文字以上にしてください。",
    "EM_ACCT_0003" => "パスワードが違います。",
    "EM_ACCT_0004" => "アカウントが存在していません。",
    "EM_ACCT_0005" => "現在のパスワードが違います。",
    "EM_ACCT_0006" => "変更後のパスワードの文字数を8文字以上にしてください。",
    "EM_THD_0001" => "スレッドが更新されております。ご確認下さい。",
};

pub struct ErrorMsg{}

pub trait GetErrorMsg {
    //keyからエラーメッセージを取得
    fn get_error_msg(&self,error_key: String) -> String;
    //key,place_holderからエラーメッセージを取得
    fn get_error_msg_by_place_holder(&self,error_key: String, place_holder: String) -> String;
}

impl GetErrorMsg for ErrorMsg {
    fn get_error_msg(&self,error_key: String) -> String{
        let error_msg = ERROR_MSG.get(&error_key).unwrap().to_string();
        return error_msg;
    }

    fn get_error_msg_by_place_holder(&self,error_key: String, place_holder: String) -> String{
        let temp_error_msg = ERROR_MSG.get(&error_key).unwrap().to_string();
        let error_msg = temp_error_msg.replace("{}", &place_holder);
        return error_msg;
    }

}