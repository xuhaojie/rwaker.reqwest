// const CONTENT_TYPE:&str = "application/x-www-form-urlencoded";

fn generate_login_authorization(user_name: &str, password: &str) -> String {
	let s = format!("{}:{}", user_name, password);
	let auth = base64::encode(s);
	auth
}

pub struct Waker {
	user_name: String,
	user_password: String,
	url: String,
	cookie: String,
	client: reqwest::Client,
}

impl Waker {
	pub fn new(url: String, user_name: String, user_password: String) -> Waker {
		let client = reqwest::Client::new();
		Waker {
			user_name,
			user_password,
			url: url,
			cookie: String::from(""),
			client,
		}
	}


	pub async fn login(self: &mut Self) -> Result<(),String> {

		let base_url = self.url.clone();
		let current_page = "Main_Login.asp";
		let next_page = "index.asp";

		let auth = generate_login_authorization(&self.user_name, &self.user_password);

		let params = [
			("group_id", ""),
			("action_mode", ""),
			("action_script", ""),
			("action_wait", "5"),
			("current_page",current_page),
			("next_page", next_page),
			("login_authorization", auth.as_str()),
		];

		let url = format!("{}/{}", base_url, "login.cgi");
		let referer = format!("{}/{}", base_url, current_page);

		let mut headers = reqwest::header::HeaderMap::new();
		headers.insert(reqwest::header::REFERER, referer.parse().unwrap());// 必选项
		headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".parse().unwrap());  //可选项
		headers.insert(reqwest::header::ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());  //可选项
		headers.insert(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2".parse().unwrap());  //可选项
		headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());  //可选项
//		headers.insert(reqwest::header::HOST ,"192.168.5.1".parse().unwrap());  //可选项
		headers.insert(reqwest::header::ORIGIN, base_url.parse().unwrap());  //可选项
		headers.insert("Upgrade-Insecure-Requests" ,"1".parse().unwrap());  //可选项


		let res = self.client
			.post(url)
			.headers(headers)
			.form(&params)
			.send()
			.await;


		match res {
			Ok(r) => {
				let h = r.headers().get("Set-Cookie");
				if let Some(v) = h {

					if let Ok(m) = v.to_str(){
						let v: Vec<&str> = m.split(';').collect();
						let t: Vec<&str> = v[0].split('=').collect();
						let token = t[1].to_owned();
						self.cookie = format!("asus_token={}", token);

					}
				}
				if self.cookie.len() > 0 {
					Ok(())
				} else {
					Err("can't get token, login failed!".to_string())
				}
			},
			Err(e) => {
				Result::Err(e.to_string())
			}
		}
	}

	#[allow(dead_code)]
	async fn get(&self, page: &str) -> Result<String,String> {
		let current_page = "Main_Login.asp";

		let url = format!("{}/{}", self.url, page);
		let referer = format!("{}/{}", self.url, current_page);

		let mut headers = reqwest::header::HeaderMap::new();
		headers.insert(reqwest::header::REFERER, referer.parse().unwrap());// 必选项
		headers.insert(reqwest::header::COOKIE, self.cookie.clone().parse().unwrap()); //必选项
		// headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());  //可选项
		// //headers.insert(reqwest::header::HOST ,"192.168.5.1".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ORIGIN, self.url.parse().unwrap());  //可选项
		// headers.insert("Upgrade-Insecure-Requests" ,"1".parse().unwrap());  //可选项

		let res = self.client
			.get(url)
			.headers(headers)
			.send()
			.await;

		match res {
			Ok(r) => match r.text().await {
				Ok(body) => Ok(body),
				Err(e) => Err(e.to_string()),
			},
			Err(e) => Err(e.to_string())
		}
	}


	pub async fn execute_command(&self, cmd: &str) -> Result<(), String> {
		let current_page = "Main_WOL_Content.asp";
		let next_page = "Main_WOL_Content.asp";

		let params = [
//			("group_id", ""),
			("action_mode", " Refresh "),
//			("action_script", ""),
//			("action_wait", ""),

			("current_page",current_page),
			("next_page", next_page),
//			("modified", ""),
//			("preferred_lang", ""),
//			("destIP", ""),
//			("firmver", "3.0.0.4"),
//			("first_time", ""),
			("SystemCmd", cmd),
//			("wollist_macAddr", ""),
		];

		let referer = format!("{}/{}", self.url, current_page);
		let url = format!("{}/{}", self.url, "apply.cgi");

		let mut headers = reqwest::header::HeaderMap::new();
		headers.insert(reqwest::header::REFERER, referer.parse().unwrap());// 必选项
		headers.insert(reqwest::header::COOKIE, self.cookie.clone().parse().unwrap()); //必选项
		// headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());  //可选项
		// //headers.insert(reqwest::header::HOST ,"192.168.5.1".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ORIGIN, base_url.parse().unwrap());  //可选项
		// headers.insert("Upgrade-Insecure-Requests" ,"1".parse().unwrap());  //可选项

		let res = self.client
			.post(url)
			.headers(headers)
			.form(&params)
			.send()
			.await;

		match res {
			Ok(_) => Ok(()),
			Err(e) => Err(e.to_string())
		}
	}

	pub async fn logout(&self) -> Result<(),String> {
		let current_page = "Main_WOL_Content.asp";
//		let next_page = "Main_WOL_Content.asp";

		let referer = format!("{}/{}", self.url, current_page);
		let url = format!("{}/{}", self.url, "logout.asp");

		let mut headers = reqwest::header::HeaderMap::new();
		headers.insert(reqwest::header::REFERER, referer.parse().unwrap());// 必选项
		headers.insert(reqwest::header::COOKIE, self.cookie.clone().parse().unwrap()); //必选项
		// headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());  //可选项
		// //headers.insert(reqwest::header::HOST ,"192.168.5.1".parse().unwrap());  //可选项
		// headers.insert(reqwest::header::ORIGIN, self.url.parse().unwrap());  //可选项
		// headers.insert("Upgrade-Insecure-Requests" ,"1".parse().unwrap());  //可选项

		let res = self.client
			.get(url)
			.headers(headers)
			.send()
			.await;

		match res {
			Ok(_) => Ok(()),
			Err(e) => Err(e.to_string()),
		}
	}
}
