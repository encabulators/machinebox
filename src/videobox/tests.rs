extern crate mockito;

use self::mockito::{mock, SERVER_URL};
use super::{Videobox, CheckOptionsBuilder};

#[test]
fn check_url() {
    let vb = Videobox::new(SERVER_URL);
    let mock = mock("POST", "/videobox/check")
        .with_body(r#"{
            "success": true,
            "id": "video-id"
        }"#)
        .create();
    {
        let opts = CheckOptionsBuilder::new()
            .result_duration("1h0m0s")
            .skip_frames(2)
            .skip_seconds(3)
            .frame_width(100)
            .frame_height(120)
            .facebox_threshold(0.75)
            .tagbox_threshold(0.7)
            .nudebox_threshold(0.2)
            .finish();

        let res =
            vb.check_url("https://test.machinebox.io/image1.png", opts);
        assert!(res.is_ok());
    }
    mock.assert();
}

#[test]
fn delete() {
    let vb = Videobox::new(SERVER_URL);
    let mock = mock("DELETE", "/videobox/results/5a50b8067eced76bad103c53dd0f5226")
        .with_status(200)
        .create();
    {
        let res = vb.delete("5a50b8067eced76bad103c53dd0f5226");
        assert!(res.is_ok());
    }
    mock.assert();
}

#[test]
fn results() {
    let vb = Videobox::new(SERVER_URL);
    let mock = mock("GET", "/videobox/results/5a50b8067eced76bad103c53dd0f5226")
        .with_status(200)
        .with_body(RESULTS_PAYLOAD)
        .create();
    {
        let res = vb.results("5a50b8067eced76bad103c53dd0f5226");
        assert!(res.is_ok());
        let analysis = res.unwrap();
        assert_eq!(analysis.facebox.unwrap().faces.len(), 1);
        assert_eq!(analysis.tagbox.unwrap().tags.len(), 3);
    }
    mock.assert();
}

#[test]
fn status() {
    let vb = Videobox::new(SERVER_URL);
    let mock = mock("GET", "/videobox/status/5a50b8067eced76bad103c53dd0f5226")
        .with_body(r#"{
            "success": true,
            "id": "5a50b8067eced76bad103c53dd0f5226",
            "status": "processing"
            }"#
        )
        .create();
    {
        let res = vb.status("5a50b8067eced76bad103c53dd0f5226");
        assert!(res.is_ok());
        let video = res.unwrap();
        assert_eq!(video.id, "5a50b8067eced76bad103c53dd0f5226");
        assert_eq!(video.status, super::Status::Processing);
    }
    mock.assert();
}

const RESULTS_PAYLOAD: &'static str = r#"
 {
	"success": true,
	"ready": true,
	"facebox": {
		"faces": [
			{
				"key": "Unknown faces",
				"instances": [
					{
						"start": 24,
						"end": 144,
						"start_ms": 1000,
						"end_ms": 6006
					},
					{
						"start": 336,
						"end": 528,
						"start_ms": 14013,
						"end_ms": 22022
					},
					{
						"start": 720,
						"end": 720,
						"start_ms": 30029,
						"end_ms": 30029
					}
				]
			}
		],
		"errorsCount": 0
	},
	"tagbox": {
		"tags": [
			{
				"key": "candle",
				"instances": [
					{
						"start": 168,
						"end": 168,
						"start_ms": 7006,
						"end_ms": 7006
					},
					{
						"start": 216,
						"end": 216,
						"start_ms": 9009,
						"end_ms": 9009
					},
					{
						"start": 312,
						"end": 312,
						"start_ms": 13012,
						"end_ms": 13012
					}
				]
			},
			{
				"key": "crutch",
				"instances": [
					{
						"start": 504,
						"end": 504,
						"start_ms": 21021,
						"end_ms": 21021
					}
				]
			},
			{
				"key": "miniskirt",
				"instances": [
					{
						"start": 72,
						"end": 72,
						"start_ms": 3003,
						"end_ms": 3003
					}
				]
			}
		],
		"errorsCount": 0
	},
	"nudebox": {
		"nudity": [
			{
				"key": "greater than 0.5 chance of nuditiy",
				"instances": [
					{
						"start": 264,
						"end": 312,
						"start_ms": 11011,
						"end_ms": 13012
					},
					{
						"start": 360,
						"end": 360,
						"start_ms": 15014,
						"end_ms": 15014
					},
					{
						"start": 408,
						"end": 408,
						"start_ms": 17017,
						"end_ms": 17017
					},
					{
						"start": 456,
						"end": 552,
						"start_ms": 19019,
						"end_ms": 23023
					},
					{
						"start": 720,
						"end": 720,
						"start_ms": 30029,
						"end_ms": 30029
					}
				]
			}
		],
		"errorsCount": 0
	}
}"#;