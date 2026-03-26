// alloc::string moduleらしい
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

// derive　attribute is diff type debug
#[derive(Debug, Clone, PartialEq)]

// 構造体
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpath: String,
}

// impl:implementation(実装)
impl Url {
    // 入力されたurlをフィールドに保存。他要素は、空文字にしてる
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpath: "".to_string(),
        }
    }

    // http:を含んでないstringはfalseを返却する。
    fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }

    // Rustでは、selfは、インスタンス自身を指す特別な引数。
    // jsでいうところのthis
    pub fn parse(&mut self) -> Result<Self, String> {
        // check sheme
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string()); // falseを返してる。
        }
        self.host = self.extract_host();
        Ok(self.clone())
    }

    // URLからホストを取得する
    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            // http://"を除去
            .trim_start_matches("http://")
            // TODO:適切なメソッドじゃないから弾かれた?→  .split(2, "/")
            // 最初の"/"までを除去
            // 最初のスラッシュが存在しない場合、つまりURLのパスとクエリパラメータが存在しない場合は、url_partsのベクタの長さは1になる。
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart:Vec<&str>;

        // find(rustの組み込みメソッド？らしい。) host部分にport Numberが含まれているかを調べる。
        // ! ポート番号を調べるらしい。がどうやって調べているのかよくわかってない。
        if let Some(index) = url_parts[0].find(":") {
            url_parts[0][..index].to_string()
        } else {
            // ポート番号が存在しない場合、デフォルトポート80をを返却
            "80".to_string()
            // url_parts[0].to_string()
        }
    }
}
