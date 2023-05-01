import requests

a = requests.get("https://barnaul.parosigara.com/catalog/zhidkost/zhidkosti_rf/zhidkost_duall_tandem_salt/zhidkost_duall_tandem_hard_evkalipt_yagody_moroznyy_energetik_2x10ml_20mg/")
# productStoreAmount.php
print(a.text)