pub fn usage() {
    println!(

r"Usage: SignTransaction [OPTIONS]

Options:
    --help              このメッセージを表示する
    --json <JSON File>  JSONファイルからパラメータの設定
    --to <ADDRESS>      送金先のアカウント、HEX文字列、ex. 0x1234567890ABCD...
    --value <VALUE>     送金金額、OCT文字列、_可、ex 1234567, 123_456_789
    --gas <GAS>         手数料、OCT文字列、_可、ex 1234567, 123_456_789
    --gas_price <PRICE> 基本料金、OCT文字列、_可、ex 1234567, 123_456_789
    --nonce <NONCE>     ナンス、OCT文字列、_可、ex 1234567, 123_456_789
    --chain <ID>        ネットワークの識別ID、OCT文字列、_可、ex 1234567, 123_456_789
    --pk <PRIVATEKEY>   送金元のキー

    JSON fileに使われる項目リスト：
        'to': '<ADDRESS>',
        'value': '<VALUE>',
        'gas': '<GAS>',
        'gas_price': '<GAS_PRICE>',
        'nonce': '<NONCE>',
        'chain': '<ID>',
    
    各項目の意味について、 --xxxにご参照
    前記の項目以外を無視される

説明
    同じ設定項目は、複数設定することが可能で、最後の値が使われる。
    例：
        SignTransaction --to 0x1111111111111111111111111 --to 0x2222222222222222222222222
        送信先のアドレスは、0x2222222222222222222222222になります。
    
    値がなしの項目は、無視される
    例：
        SignTransaction --to --nonce 1000
        --toが無視される
");
}
