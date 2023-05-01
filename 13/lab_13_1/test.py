import requests

param = {
    'USE_STORE_PHONE': 'Y', 
    'SCHEDULE': '', 
    'USE_MIN_AMOUNT': 'N', 
    'MIN_AMOUNT': '2', 
    'ELEMENT_ID': 30584, 
    'STORE_PATH': '/contacts/stores/#store_id#/', 
    'MAIN_TITLE': 'Наличие в магазинах', 
    'MAX_AMOUNT': '20', 
    'USE_ONLY_MAX_AMOUNT': 'Y', 
    'SHOW_EMPTY_STORE': 'Y', 
    'SHOW_GENERAL_STORE_INFORMATION': 'N', 
    'USER_FIELDS': ['', ''], 
    'FIELDS': ['TITLE', 'ADDRESS', ''], 
    'STORES_FILTER_ORDER': 'SORT_ASC', 
    'STORES_FILTER': 'TITLE', 
    'STORES': ['10', '30'], 
    'SET_ITEMS': '',
    'SITE_ID': 's1', 
    'USE_STORES': True
}


def f():
    a = requests.post("https://barnaul.parosigara.com/ajax/productStoreAmount.php", params=param)
    if a.text == '':
        return f()
    else:
        return a.text
    
print(f())
