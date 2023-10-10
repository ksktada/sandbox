# iCD
# https://www.ipa.go.jp/jinzai/skill-standard/plus-it-ui/history/icd.html

# graph化
# https://qiita.com/mekadomio/items/ce304f5a394af5867775
# https://qiita.com/tomati2021/items/cf2e3ad86b95dbf0431d
# https://qiita.com/tomati2021/items/426ae2cc89099bf7ecc3

import csv
from graphviz import Digraph
from tqdm import tqdm

skill_map = {}

# shift_jis ではなく cp932 を指定
with open('./skill_list_item.csv', encoding="cp932") as f:
    reader = csv.reader(f)
    # ヘッダーをスキップ
    header = next(reader)

    # 初回の行を取得
    first_row = next(reader)

    category_pre = first_row[0]
    classification_pre = first_row[5]

    skill_map[first_row[0]] = {first_row[4]: [first_row[7]]}

    for row in reader:
        if category_pre != row[0]:
            skill_map[row[0]] = {row[4]: [row[7]]}
            category_pre = row[0]
            classification_pre = row[4]
        elif classification_pre != row[4]:
            skill_map[row[0]][row[4]] = [row[7]]
            classification_pre = row[4]
        else:
            skill_map[row[0]][row[4]].append(row[7])

    for nums, unique_name in enumerate(tqdm(skill_map.keys())):
        # formatはpngを指定(他にはPDF, PNG, SVGなどが指定可)
        G = Digraph(format="pdf", engine="circo")
        G.attr('node', shape='circle')

        # 文字化け防止のためフォントを指定
        G.attr('node', fontname = 'Meiryo UI')
        G.attr('edge', fontname = 'Meiryo UI')

        # ノードを追加
        for i, j in skill_map[unique_name].items():
            G.node(i)
            for k in j:
                if type(k) == str:
                    G.node(k)

        # 辺を追加
        for i, j in skill_map[unique_name].items():
            G.edge(unique_name, i)
            for k in j:
                if type(k) == str:
                    G.edge(i, k)

        # 画像を保存（拡張子は不要）
        G.render("./"+unique_name)
