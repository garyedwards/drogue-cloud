mod common;

use actix_web::{web, App};
use drogue_cloud_authentication_service::{endpoints, service, WebData};
use drogue_cloud_service_api::auth::device::authn::{AuthenticationRequest, Credential};
use drogue_cloud_test_common::{client, db};
use serde_json::{json, Value};
use serial_test::serial;

#[allow(dead_code)]
const DEVICE1_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
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
my4gvNoJcsgq35PsyQyJp2CpCTXvnwM=
-----END PRIVATE KEY-----
"#;

#[allow(dead_code)]
const DEVICE1_CRT: &str = r#"
-----BEGIN CERTIFICATE-----
MIIFUDCCAzigAwIBAgIUVHs1bGiLZY47Go8xCFuZepQT+lswDQYJKoZIhvcNAQEL
BQAwPTETMBEGA1UECgwKRHJvZ3VlIElvVDEOMAwGA1UECwwFQ2xvdWQxFjAUBgNV
BAMMDUFwcGxpY2F0aW9uIDEwHhcNMjEwMjAyMDgzOTE3WhcNMzEwMTMxMDgzOTE3
WjA4MRMwEQYDVQQKDApEcm9ndWUgSW9UMQ4wDAYDVQQLDAVDbG91ZDERMA8GA1UE
AwwIRGV2aWNlIDEwggIiMA0GCSqGSIb3DQEBAQUAA4ICDwAwggIKAoICAQDwtttE
okszRFqIpUoMTPzDgHW6fsx2Kn5CKnBz1aCskWwt6ML7waEKloPlJ4426qxaYmfc
N8iKeMLuZnDR4e1qK2M2cXO24NJpe878PXMKn87y7mdyH0JxsqIiNgZTJ6zyZNXw
A61vr7NhMTvSe7xGSN2ii6EupK6UdeHiLjKxxk3dz3RDLC+u6gEJLR4xwo4su383
MVEjIMrO0qf34/lx0Hez7jLXjXPaWAkBqhL2/sWKGrnFV9rc+3xUFjvHeTJUWDVG
3vmSvo09asRZwQFOv1baVQCdMcicp8xmnSIKmjuEnFwbAmYSJushV9XQiwZkj8c1
ozQOXLEq+gMEEe5HhtpJbPkSn0lnuCmtexPBr7N3FPR39wwBWdrzpWPEcUD4UGHk
2yBxl+EOjTops7ZbiD7bYwne3RW4O0P3pj8sadaW+TZaBMf9XmY56fG+NWLUwysw
yyubcIsAVy/Y/5Ti5HVSGjWmA2/IYh2XaLW3tsOG6/yfkZz9ICsNTGg3LL/mtR/R
955JyJl2K66lLRkgiQOSSwqnTfohVx2wUfXAnkxEa24FK9kGjDv4UtiWo45MB3hn
h2ij6VddqGwa1TD0fOiy8Cn4wEXd8OudXEz83j/8uRX/5yDjGPsY53ZsJ2P959SK
ak4hF7J5z7qDHMS6ODH/dYOZXwVNBeftbLPaoQIDAQABo00wSzAdBgNVHQ4EFgQU
DnXh2f0zzDZeTTKKDJuWX/MaJWYwCwYDVR0PBAQDAgOoMB0GA1UdJQQWMBQGCCsG
AQUFBwMBBggrBgEFBQcDAjANBgkqhkiG9w0BAQsFAAOCAgEABDsFioMYbwXkNYMU
7jrQZhxBHo0EqV61e3ESpVrdco/EyunZl5CIwYlT7XgrJ1hsSXXQQh8lvzAG8Bz+
5X8dAQnzZzZat8oBtkRFIlOnCQZ1baF/ZqE8HIHs6IOQC9aHJn3/iNklXWk4B8T3
NmBWsbT16DyoUyX91nlRMOCRSOgrEPNqgD2D3k1Wk3JXzjJGWhet2FoBWrH9Fx0N
lwZbWgOU+hRfdsC5++ACqRPxWps0IHOLoNjWPK1Jhrz53U4Mg5I/hZ/qEFYP0TTC
lHP1G+qu2nNfANCpz/uoYza7qDuX1vwtuj9j19Rcrpe47fY68GJuK7k7cj+Ynaie
RYXPUKNwYs/qRphxl1JD4sUdkrgLyK9OP+AM450BjMAgPFuUWQJ5t620MPJY/ueu
Shwx6p7TkXaLwlOi0Y8QwqiNcp4rGXOzO2FAON/K51zbF260cD7QZU73aJJMEfyb
EtCx9GcRnF3POyCYgmAC2T/rRKAvhBX1IeEqVtvyaC5+Vx//wKpimRfXWgUNJa+b
qJGSHehPBR9rz5QtIPMP8uyaIc3tE9UpbOhbTcZ1xhAa/1s6O+h4geBO1X4Qs1q4
paoOuJpVUal+hf/7Q2B9P/E6UADJIblfqIsPSK/4lfhIUFLQz5iRjpXJ+bd+gXiQ
g8An1ghg/KzWtefInZ2vr4+Kv5w=
-----END CERTIFICATE-----
"#;

#[allow(dead_code)]
const DEVICE1_FULLCHAIN_CRT: &str = r#"
-----BEGIN CERTIFICATE-----
MIIFUDCCAzigAwIBAgIUVHs1bGiLZY47Go8xCFuZepQT+lswDQYJKoZIhvcNAQEL
BQAwPTETMBEGA1UECgwKRHJvZ3VlIElvVDEOMAwGA1UECwwFQ2xvdWQxFjAUBgNV
BAMMDUFwcGxpY2F0aW9uIDEwHhcNMjEwMjAyMDgzOTE3WhcNMzEwMTMxMDgzOTE3
WjA4MRMwEQYDVQQKDApEcm9ndWUgSW9UMQ4wDAYDVQQLDAVDbG91ZDERMA8GA1UE
AwwIRGV2aWNlIDEwggIiMA0GCSqGSIb3DQEBAQUAA4ICDwAwggIKAoICAQDwtttE
okszRFqIpUoMTPzDgHW6fsx2Kn5CKnBz1aCskWwt6ML7waEKloPlJ4426qxaYmfc
N8iKeMLuZnDR4e1qK2M2cXO24NJpe878PXMKn87y7mdyH0JxsqIiNgZTJ6zyZNXw
A61vr7NhMTvSe7xGSN2ii6EupK6UdeHiLjKxxk3dz3RDLC+u6gEJLR4xwo4su383
MVEjIMrO0qf34/lx0Hez7jLXjXPaWAkBqhL2/sWKGrnFV9rc+3xUFjvHeTJUWDVG
3vmSvo09asRZwQFOv1baVQCdMcicp8xmnSIKmjuEnFwbAmYSJushV9XQiwZkj8c1
ozQOXLEq+gMEEe5HhtpJbPkSn0lnuCmtexPBr7N3FPR39wwBWdrzpWPEcUD4UGHk
2yBxl+EOjTops7ZbiD7bYwne3RW4O0P3pj8sadaW+TZaBMf9XmY56fG+NWLUwysw
yyubcIsAVy/Y/5Ti5HVSGjWmA2/IYh2XaLW3tsOG6/yfkZz9ICsNTGg3LL/mtR/R
955JyJl2K66lLRkgiQOSSwqnTfohVx2wUfXAnkxEa24FK9kGjDv4UtiWo45MB3hn
h2ij6VddqGwa1TD0fOiy8Cn4wEXd8OudXEz83j/8uRX/5yDjGPsY53ZsJ2P959SK
ak4hF7J5z7qDHMS6ODH/dYOZXwVNBeftbLPaoQIDAQABo00wSzAdBgNVHQ4EFgQU
DnXh2f0zzDZeTTKKDJuWX/MaJWYwCwYDVR0PBAQDAgOoMB0GA1UdJQQWMBQGCCsG
AQUFBwMBBggrBgEFBQcDAjANBgkqhkiG9w0BAQsFAAOCAgEABDsFioMYbwXkNYMU
7jrQZhxBHo0EqV61e3ESpVrdco/EyunZl5CIwYlT7XgrJ1hsSXXQQh8lvzAG8Bz+
5X8dAQnzZzZat8oBtkRFIlOnCQZ1baF/ZqE8HIHs6IOQC9aHJn3/iNklXWk4B8T3
NmBWsbT16DyoUyX91nlRMOCRSOgrEPNqgD2D3k1Wk3JXzjJGWhet2FoBWrH9Fx0N
lwZbWgOU+hRfdsC5++ACqRPxWps0IHOLoNjWPK1Jhrz53U4Mg5I/hZ/qEFYP0TTC
lHP1G+qu2nNfANCpz/uoYza7qDuX1vwtuj9j19Rcrpe47fY68GJuK7k7cj+Ynaie
RYXPUKNwYs/qRphxl1JD4sUdkrgLyK9OP+AM450BjMAgPFuUWQJ5t620MPJY/ueu
Shwx6p7TkXaLwlOi0Y8QwqiNcp4rGXOzO2FAON/K51zbF260cD7QZU73aJJMEfyb
EtCx9GcRnF3POyCYgmAC2T/rRKAvhBX1IeEqVtvyaC5+Vx//wKpimRfXWgUNJa+b
qJGSHehPBR9rz5QtIPMP8uyaIc3tE9UpbOhbTcZ1xhAa/1s6O+h4geBO1X4Qs1q4
paoOuJpVUal+hf/7Q2B9P/E6UADJIblfqIsPSK/4lfhIUFLQz5iRjpXJ+bd+gXiQ
g8An1ghg/KzWtefInZ2vr4+Kv5w=
-----END CERTIFICATE-----
-----BEGIN CERTIFICATE-----
MIIFRzCCAy+gAwIBAgIUDVv6CIk0s1nHEJjGKWhNuLMGY6YwDQYJKoZIhvcNAQEL
BQAwPTETMBEGA1UECgwKRHJvZ3VlIElvVDEOMAwGA1UECwwFQ2xvdWQxFjAUBgNV
BAMMDUFwcGxpY2F0aW9uIDEwHhcNMjEwMjAyMDgzOTE3WhcNMzEwMTMxMDgzOTE3
WjA9MRMwEQYDVQQKDApEcm9ndWUgSW9UMQ4wDAYDVQQLDAVDbG91ZDEWMBQGA1UE
AwwNQXBwbGljYXRpb24gMTCCAiIwDQYJKoZIhvcNAQEBBQADggIPADCCAgoCggIB
AMHZw/PKRRDf6JknsN3CjgZhy0cvLRMkHwqD4v8wy+sfIZod6KE0SQuEc60COOlP
dXu8iBSGC47ww4ESlPB7ATxVCS18yXeAJnRiPqv5FOzgo4niL18FM9ONAM2pPHkz
H2XR6g4RCS6CPQNz+p8d55RcpkdwRZbufnx/rXyhrwDoyN2EA7pxcbOI9DlL64HM
P1DxSHGhAN5RKkS2Jer1xububwdJdrivyb8HWm/BRo6JkFceEJZeoXILWD+m16ee
uZr1xP4vpahTyrbqevAImuh0ErT7AWC5Z6rLYDyAMuFgsYmvNCo8wd6vKqI4WXOX
MBNxcvJuQhc4oNafndoEdbsz3Zo6sYhLIMKGnVLeHoMJB8YdzKUt0vhz6JcU2hBT
cjiNPL4NfFRUdg874q7rDQTUMqqCPJIrQ3bagJiGcRJFJ0YunKI5OftjqALBj3hs
96MTKbj/vElHXfYM+uKa4ksUbfmZ0wL5WiHPmv4BtrsrXPuQX0Ibzek2uJlgb0WZ
dZapzRNYm/DejZqpM3MENVlnvm/ocKArTO3fyqrCEqncJzmzH9XrH7jUWN+pejA/
x7/L/c+4Abmc85v5VTcPq04+2In9foIGuIN4UVc52GNV3jzU8tZrvPzsss7LlIio
oHpEwOFzcJ5uPus5xCTXs1/TUBo0xgmoPxC5mdnFxX3lAgMBAAGjPzA9MB0GA1Ud
DgQWBBTj/bEg63kQiLFWWlVcsD/93/6y/TAPBgNVHRMBAf8EBTADAQH/MAsGA1Ud
DwQEAwIBBjANBgkqhkiG9w0BAQsFAAOCAgEAO/ZwQjjfPrWIVsNcBD6Xeu0+5/KQ
44DOyIdpksqbjTu6Ynh60s46Jjfv0++RyPBx2YQCT1vaHYw7nqM8m5f97t9diuHB
BCQv+Xzhkv+VtrEgkp5/QQhrd0jOBJzY6eNuwN+gn79L42K7f983M+Qu98KFTSWx
gsUg2QPgFdh7btBmP7Mj7FBwXgI5J+heZZkkdmF/2ftew2+55gqPUOp5WBMKh0ze
FHAcOxm0WhNpM9cA13FcuoAKrWbapmCgx6Gyl2cEb4EJ8L8/eAC2mVR+9Wp6Fxe7
nDMSBihk1VWmvA7E4jrAyOS3gN+QJbNnxnZCYIvfmW0S0bNhWUPJJMcLEZTKWi6R
GU5ROLuF+luPixmke8ikExUyA+N9PygjSuQ/WtAvKLpQ0ZRCuFjIq5+I7tgnn6e8
AkdGp9Onit0wyMigTmyIojQOA0Ke4mx4COJhac5PBF82I1Ti5TrgjwchHVnQSy0r
+kWtI1UodQj+z+krYbjrjcvfubAVG2Lw4Bt8R2jDMnYKyJ1hZ6THHKxvTC4y7YEy
guNkCPchuk58ji4KdghFFZXOaA6Vj563Jf9CVj20RNlCUW+d+DAttzv+HWIM+VWE
as/acK9YHdRd34e3ZP5f2xbtQMHl5Y+xxKv0iLVO54USApmM1ME6EuCVUKwscmMp
/rR+CFcZQGdI+fk=
-----END CERTIFICATE-----
"#;

const DEVICE1_CRT_BAD: &str = r#"
-----BEGIN CERTIFICATE-----
MIIFUDCCAzigAwIBAgIUS33aHoMeMHUAL0FbpTmvWxyePy0wDQYJKoZIhvcNAQEL
BQAwPTETMBEGA1UECgwKRHJvZ3VlIElvVDEOMAwGA1UECwwFQ2xvdWQxFjAUBgNV
BAMMDUFwcGxpY2F0aW9uIDEwHhcNMjEwMjAyMTExMTMxWhcNMzEwMTMxMTExMTMx
WjA4MRMwEQYDVQQKDApEcm9ndWUgSW9UMQ4wDAYDVQQLDAVDbG91ZDERMA8GA1UE
AwwIRGV2aWNlIDEwggIiMA0GCSqGSIb3DQEBAQUAA4ICDwAwggIKAoICAQDP7kv1
jc/AFJaqV8E9nVB8ts912WkWekXXTEcl6C0zlFtaPuO0rnE8jSHOJQLOPglKdfF5
+w115W1/TK1uZQWS8VuuMUTebrtUO0fw19XMJYjxg6SV5fB/596oiAKNwrb8CP5s
p8CJyytYO7BdGotmxAhABWXHWd8gOFJMtlksl+hAkHSVybgdih4ng4QEPj9Fq+L1
U6G4XBVBxNpDFXQl8Dar9zRY6txi83uBQ8ykEfBIo5X2qimRdLdetZsxh0K3VrqY
EJDQaLslb4gPxQZCB7BFnIkOry7aD7SbXTFkEsHv+oWnSuTVkKkGHRbko2PW3lJm
hV1Lst6ZcXm/ekux7H7sr8I80N16Peb9DtZbTi9DqBN7VOfPgSXJRb09EqZi5zlV
QmLLnIgI2mjb7jC/Tk87i+XYJKM/PX4rmbp6j3CMa96VQ8OLrG0AYFkaLPThgu/k
AxmZXHHEtX3uZrfikX3ke6Ry5iNGQATqa2Ky6/8T5r3jXBUW8TxKtfh4thgJnw76
F9+6r1RppBSuXXpm7prd/ZPT5wy68z3P9aN+EYC7fFwjfFUnaN6OMVOA+9keY2l8
eXGYCVGtEjHyHCXFVyBhRtY8aNQ6xDRR3eXhZ40T4X5GpG4ugz/70eSz/l/4Herr
N6wV3Yncwav9GL205zwGXbQ/Z1P7dp4MlWkV7wIDAQABo00wSzAdBgNVHQ4EFgQU
D/1i2/f8zFMUae5ZOWfSRlJ8y+swCwYDVR0PBAQDAgOoMB0GA1UdJQQWMBQGCCsG
AQUFBwMBBggrBgEFBQcDAjANBgkqhkiG9w0BAQsFAAOCAgEAAAcn44d5L3JKKHoU
iXJ6lQLcJJLcs8T8Ox6pd/ErLcKKupkOHOmjN4D6brqDo753DUOh6L64aHRNq1CU
E4KkbBtAsEvWFa/T6d0Tt/8idY+V1Vq8sk1p0k2L2G4kgAzVJEnhWBJHpxD9Lfoo
7YATNUEAD5bIFQifjYrjI18He7gLNTpU/oQv9QCE1Ou98DClv0aYpE8cDHm+RikN
GVA410Hr6prmbhy3zcvnliNGrIf2FiWRkrsrOn0Gdei4Ueu4qpSNUvVwyXaXwHbp
2dq/8tTp71B4GCJtKMPNQbZO5S63q6iNjiQMmEW3PsrShAaueCoZ5FEK5/Lp4urd
eQFJefisF68jDTAkZvWSUDAScb4YzzVro0MUbl20QGLs2m9ZnLiyGAjC6o1Oposx
g4g/LWmujVzGPTUL/W4Co5dou1gZuTF4Olp5DHeCyxlnyToBcHGAV2RGO/8+FYym
fVrCkOgLu30JyyYBPn/PFJTvbHXSiOvs+QbgqJDkutaLeKEZP4FDE+bcbHW41IH6
Qq8COL59SUIoY+VkpYRMLjKRzdYcezHTSRpr4c7F1dhcfmtAR1Y8Yg3xpEUCrUBw
3HPoM7PEhIUEaH2twa7p+ZjCWECyhGepHJg6b3sykyijQLPV+HBvfP5PDipCj9M9
B+IWM/nWYN5yEEYHRfPdbgP1clg=
-----END CERTIFICATE-----
"#;

fn device1_json() -> Value {
    json!({"pass":{
        "application": {
            "metadata": {
                "name": "app2",
                "uid": "4e185ea6-7c26-11eb-a319-d45d6455d220",
                "creationTimestamp": "2020-01-01T00:00:00Z",
                "resourceVersion": "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
                "generation": 0,
            },
            "spec": {
                "trustAnchors": {
                    "anchors": [
                        { "certificate": "LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUZSekNDQXkrZ0F3SUJBZ0lVRFZ2NkNJazBzMW5IRUpqR0tXaE51TE1HWTZZd0RRWUpLb1pJaHZjTkFRRUwKQlFBd1BURVRNQkVHQTFVRUNnd0tSSEp2WjNWbElFbHZWREVPTUF3R0ExVUVDd3dGUTJ4dmRXUXhGakFVQmdOVgpCQU1NRFVGd2NHeHBZMkYwYVc5dUlERXdIaGNOTWpFd01qQXlNRGd6T1RFM1doY05NekV3TVRNeE1EZ3pPVEUzCldqQTlNUk13RVFZRFZRUUtEQXBFY205bmRXVWdTVzlVTVE0d0RBWURWUVFMREFWRGJHOTFaREVXTUJRR0ExVUUKQXd3TlFYQndiR2xqWVhScGIyNGdNVENDQWlJd0RRWUpLb1pJaHZjTkFRRUJCUUFEZ2dJUEFEQ0NBZ29DZ2dJQgpBTUhady9QS1JSRGY2Smtuc04zQ2pnWmh5MGN2TFJNa0h3cUQ0djh3eStzZklab2Q2S0UwU1F1RWM2MENPT2xQCmRYdThpQlNHQzQ3d3c0RVNsUEI3QVR4VkNTMTh5WGVBSm5SaVBxdjVGT3pnbzRuaUwxOEZNOU9OQU0ycFBIa3oKSDJYUjZnNFJDUzZDUFFOeitwOGQ1NVJjcGtkd1JaYnVmbngvclh5aHJ3RG95TjJFQTdweGNiT0k5RGxMNjRITQpQMUR4U0hHaEFONVJLa1MySmVyMXh1YnVid2RKZHJpdnliOEhXbS9CUm82SmtGY2VFSlplb1hJTFdEK20xNmVlCnVacjF4UDR2cGFoVHlyYnFldkFJbXVoMEVyVDdBV0M1WjZyTFlEeUFNdUZnc1ltdk5Dbzh3ZDZ2S3FJNFdYT1gKTUJOeGN2SnVRaGM0b05hZm5kb0VkYnN6M1pvNnNZaExJTUtHblZMZUhvTUpCOFlkektVdDB2aHo2SmNVMmhCVApjamlOUEw0TmZGUlVkZzg3NHE3ckRRVFVNcXFDUEpJclEzYmFnSmlHY1JKRkowWXVuS0k1T2Z0anFBTEJqM2hzCjk2TVRLYmovdkVsSFhmWU0rdUthNGtzVWJmbVowd0w1V2lIUG12NEJ0cnNyWFB1UVgwSWJ6ZWsydUpsZ2IwV1oKZFphcHpSTlltL0RlalpxcE0zTUVOVmxudm0vb2NLQXJUTzNmeXFyQ0VxbmNKem16SDlYckg3alVXTitwZWpBLwp4Ny9ML2MrNEFibWM4NXY1VlRjUHEwNCsySW45Zm9JR3VJTjRVVmM1MkdOVjNqelU4dFpydlB6c3NzN0xsSWlvCm9IcEV3T0Z6Y0o1dVB1czV4Q1RYczEvVFVCbzB4Z21vUHhDNW1kbkZ4WDNsQWdNQkFBR2pQekE5TUIwR0ExVWQKRGdRV0JCVGovYkVnNjNrUWlMRldXbFZjc0QvOTMvNnkvVEFQQmdOVkhSTUJBZjhFQlRBREFRSC9NQXNHQTFVZApEd1FFQXdJQkJqQU5CZ2txaGtpRzl3MEJBUXNGQUFPQ0FnRUFPL1p3UWpqZlByV0lWc05jQkQ2WGV1MCs1L0tRCjQ0RE95SWRwa3NxYmpUdTZZbmg2MHM0NkpqZnYwKytSeVBCeDJZUUNUMXZhSFl3N25xTThtNWY5N3Q5ZGl1SEIKQkNRditYemhrditWdHJFZ2twNS9RUWhyZDBqT0JKelk2ZU51d04rZ243OUw0Mks3Zjk4M00rUXU5OEtGVFNXeApnc1VnMlFQZ0ZkaDdidEJtUDdNajdGQndYZ0k1SitoZVpaa2tkbUYvMmZ0ZXcyKzU1Z3FQVU9wNVdCTUtoMHplCkZIQWNPeG0wV2hOcE05Y0ExM0ZjdW9BS3JXYmFwbUNneDZHeWwyY0ViNEVKOEw4L2VBQzJtVlIrOVdwNkZ4ZTcKbkRNU0JpaGsxVldtdkE3RTRqckF5T1MzZ04rUUpiTm54blpDWUl2Zm1XMFMwYk5oV1VQSkpNY0xFWlRLV2k2UgpHVTVST0x1RitsdVBpeG1rZThpa0V4VXlBK045UHlnalN1US9XdEF2S0xwUTBaUkN1RmpJcTUrSTd0Z25uNmU4CkFrZEdwOU9uaXQwd3lNaWdUbXlJb2pRT0EwS2U0bXg0Q09KaGFjNVBCRjgySTFUaTVUcmdqd2NoSFZuUVN5MHIKK2tXdEkxVW9kUWoreitrcllianJqY3ZmdWJBVkcyTHc0QnQ4UjJqRE1uWUt5SjFoWjZUSEhLeHZUQzR5N1lFeQpndU5rQ1BjaHVrNThqaTRLZGdoRkZaWE9hQTZWajU2M0pmOUNWajIwUk5sQ1VXK2QrREF0dHp2K0hXSU0rVldFCmFzL2FjSzlZSGRSZDM0ZTNaUDVmMnhidFFNSGw1WSt4eEt2MGlMVk81NFVTQXBtTTFNRTZFdUNWVUt3c2NtTXAKL3JSK0NGY1pRR2RJK2ZrPQotLS0tLUVORCBDRVJUSUZJQ0FURS0tLS0tCg==" }
                    ]
                }
            },
            "status": {
                "trustAnchors": {
                    "anchors": [{
                        "valid": {
                            "certificate": "LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUZSekNDQXkrZ0F3SUJBZ0lVRFZ2NkNJazBzMW5IRUpqR0tXaE51TE1HWTZZd0RRWUpLb1pJaHZjTkFRRUwKQlFBd1BURVRNQkVHQTFVRUNnd0tSSEp2WjNWbElFbHZWREVPTUF3R0ExVUVDd3dGUTJ4dmRXUXhGakFVQmdOVgpCQU1NRFVGd2NHeHBZMkYwYVc5dUlERXdIaGNOTWpFd01qQXlNRGd6T1RFM1doY05NekV3TVRNeE1EZ3pPVEUzCldqQTlNUk13RVFZRFZRUUtEQXBFY205bmRXVWdTVzlVTVE0d0RBWURWUVFMREFWRGJHOTFaREVXTUJRR0ExVUUKQXd3TlFYQndiR2xqWVhScGIyNGdNVENDQWlJd0RRWUpLb1pJaHZjTkFRRUJCUUFEZ2dJUEFEQ0NBZ29DZ2dJQgpBTUhady9QS1JSRGY2Smtuc04zQ2pnWmh5MGN2TFJNa0h3cUQ0djh3eStzZklab2Q2S0UwU1F1RWM2MENPT2xQCmRYdThpQlNHQzQ3d3c0RVNsUEI3QVR4VkNTMTh5WGVBSm5SaVBxdjVGT3pnbzRuaUwxOEZNOU9OQU0ycFBIa3oKSDJYUjZnNFJDUzZDUFFOeitwOGQ1NVJjcGtkd1JaYnVmbngvclh5aHJ3RG95TjJFQTdweGNiT0k5RGxMNjRITQpQMUR4U0hHaEFONVJLa1MySmVyMXh1YnVid2RKZHJpdnliOEhXbS9CUm82SmtGY2VFSlplb1hJTFdEK20xNmVlCnVacjF4UDR2cGFoVHlyYnFldkFJbXVoMEVyVDdBV0M1WjZyTFlEeUFNdUZnc1ltdk5Dbzh3ZDZ2S3FJNFdYT1gKTUJOeGN2SnVRaGM0b05hZm5kb0VkYnN6M1pvNnNZaExJTUtHblZMZUhvTUpCOFlkektVdDB2aHo2SmNVMmhCVApjamlOUEw0TmZGUlVkZzg3NHE3ckRRVFVNcXFDUEpJclEzYmFnSmlHY1JKRkowWXVuS0k1T2Z0anFBTEJqM2hzCjk2TVRLYmovdkVsSFhmWU0rdUthNGtzVWJmbVowd0w1V2lIUG12NEJ0cnNyWFB1UVgwSWJ6ZWsydUpsZ2IwV1oKZFphcHpSTlltL0RlalpxcE0zTUVOVmxudm0vb2NLQXJUTzNmeXFyQ0VxbmNKem16SDlYckg3alVXTitwZWpBLwp4Ny9ML2MrNEFibWM4NXY1VlRjUHEwNCsySW45Zm9JR3VJTjRVVmM1MkdOVjNqelU4dFpydlB6c3NzN0xsSWlvCm9IcEV3T0Z6Y0o1dVB1czV4Q1RYczEvVFVCbzB4Z21vUHhDNW1kbkZ4WDNsQWdNQkFBR2pQekE5TUIwR0ExVWQKRGdRV0JCVGovYkVnNjNrUWlMRldXbFZjc0QvOTMvNnkvVEFQQmdOVkhSTUJBZjhFQlRBREFRSC9NQXNHQTFVZApEd1FFQXdJQkJqQU5CZ2txaGtpRzl3MEJBUXNGQUFPQ0FnRUFPL1p3UWpqZlByV0lWc05jQkQ2WGV1MCs1L0tRCjQ0RE95SWRwa3NxYmpUdTZZbmg2MHM0NkpqZnYwKytSeVBCeDJZUUNUMXZhSFl3N25xTThtNWY5N3Q5ZGl1SEIKQkNRditYemhrditWdHJFZ2twNS9RUWhyZDBqT0JKelk2ZU51d04rZ243OUw0Mks3Zjk4M00rUXU5OEtGVFNXeApnc1VnMlFQZ0ZkaDdidEJtUDdNajdGQndYZ0k1SitoZVpaa2tkbUYvMmZ0ZXcyKzU1Z3FQVU9wNVdCTUtoMHplCkZIQWNPeG0wV2hOcE05Y0ExM0ZjdW9BS3JXYmFwbUNneDZHeWwyY0ViNEVKOEw4L2VBQzJtVlIrOVdwNkZ4ZTcKbkRNU0JpaGsxVldtdkE3RTRqckF5T1MzZ04rUUpiTm54blpDWUl2Zm1XMFMwYk5oV1VQSkpNY0xFWlRLV2k2UgpHVTVST0x1RitsdVBpeG1rZThpa0V4VXlBK045UHlnalN1US9XdEF2S0xwUTBaUkN1RmpJcTUrSTd0Z25uNmU4CkFrZEdwOU9uaXQwd3lNaWdUbXlJb2pRT0EwS2U0bXg0Q09KaGFjNVBCRjgySTFUaTVUcmdqd2NoSFZuUVN5MHIKK2tXdEkxVW9kUWoreitrcllianJqY3ZmdWJBVkcyTHc0QnQ4UjJqRE1uWUt5SjFoWjZUSEhLeHZUQzR5N1lFeQpndU5rQ1BjaHVrNThqaTRLZGdoRkZaWE9hQTZWajU2M0pmOUNWajIwUk5sQ1VXK2QrREF0dHp2K0hXSU0rVldFCmFzL2FjSzlZSGRSZDM0ZTNaUDVmMnhidFFNSGw1WSt4eEt2MGlMVk81NFVTQXBtTTFNRTZFdUNWVUt3c2NtTXAKL3JSK0NGY1pRR2RJK2ZrPQotLS0tLUVORCBDRVJUSUZJQ0FURS0tLS0tCg==",
                            "subject": "O=Drogue IoT, OU=Cloud, CN=Application 1",
                            "notBefore": "2021-02-02T08:39:17Z",
                            "notAfter": "2031-01-31T08:39:17Z"
                        }
                    }]
                }
            }
        },
        "device": {
            "metadata": {
                "application": "app2",
                "name": "device1",
                "uid": "4e185ea6-7c26-11eb-a319-d45d6455d221",
                "creationTimestamp": "2020-01-01T00:00:00Z",
                "resourceVersion": "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
                "generation": 0,
            },
        }
    }})
}

/// Decode PEM certificate chain from string and return as byte array.
fn from_pem(pem: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    let pems = pem::parse_many(pem)?;
    let mut result = Vec::with_capacity(pems.len());
    for pem in pems {
        result.push(pem.contents);
    }
    Ok(result)
}

/// Authorize a device using an X.509 client certificate (not full chain).
#[actix_rt::test]
#[serial]
async fn test_x509_client_cert() {
    let app_id = "O=Drogue IoT, OU=Cloud, CN=Application 1";
    let device_id = "O=Drogue IoT, OU=Cloud, CN=Device 1";
    test_auth!(AuthenticationRequest{
        application: app_id.into(),
        device: device_id.into(),
        credential: Credential::Certificate(from_pem(DEVICE1_CRT).unwrap()),
        r#as: None
    } => device1_json());
}

/// Authorize a device using an X.509 client certificate.
///
/// The certificate is signed by a different CA, and thus the validation must fail.
#[actix_rt::test]
#[serial]
async fn test_x509_client_cert_bad() {
    let app_id = "O=Drogue IoT, OU=Cloud, CN=Application 1";
    let device_id = "O=Drogue IoT, OU=Cloud, CN=Device 1";
    test_auth!(AuthenticationRequest{
        application: app_id.into(),
        device: device_id.into(),
        credential: Credential::Certificate(from_pem(DEVICE1_CRT_BAD).unwrap()),
        r#as: None
    } => json!("fail"));
}
