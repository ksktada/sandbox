# Kubernetes

## 参考

- Kubernetes完全ガイド

## 基礎

- Kubernetes Master
  - Kubernetesを操作するノード
  - APIエンドポイントの提供
  - コンテナのスケジューリング、スケーリング
  - など
- Kubernetes Node
  - 実際にコンテナが起動するノード

## リソース

### Workloadsリソース<br/>...クラスタ上にコンテナを起動させるために利用するリソース)

- Pod
- ReplicationController
- ReplicaSet
- Deployment
- DaemonSet
- StatefulSet
- Job
- CronJob

### Discovery & LBリソース<br/>...コンテナのサービスディスカバリや、クラスタの外部からもアクセス可能なエンドポイントなどを提供するリソース

- Service
  - ClusterIP
  - ExternalIP(ClusterIPの一種)
  - NodePort
  - LoadBalancer
  - Headless(None)
  - ExternalName
  - None-Selector
- Ingress

### Config & Storageリソース<br/>...設定や機密データをコンテナに埋め込んだり、永続ボリュームを提供するリソース

- Secret
- ConfigMap
- PersistentVolumeClaim

### Clusterリソース<br/>...クラスタ自体の振る舞いを定義するリソース

- Node
- Namespace
  - kube-system...KubernetesクラスタのコンポーネントやアドオンがデプロイされるNamespace(例: Kubernetes Dashboard)
  - kube-public...全ユーザが利用できるConfigMapなどを配置するNamespace
  - default...デフォルトのNamespace
    - 目的に応じて任意のNamespaceを作成する。1つのクラスタを共有利用したり、システムが複雑でない場合はdefaultを使用してもOK
    - NamespaceとRBAC(Role-Based Access Control)とNetworkPolicyを組み合わせることで分離性を高めることができる
      - アプリケーション開発者は`kube-system`、`kube-public`を触れないようにする、など
- PersistentVolume
- ResourceQuota
- ServiceAccount
- Role
- ClusterRole
- RoleBinding
- ClusterRoleBinding
- NetworkPolicy

### Metaリソース<br/>...クラスタ内の他のリソースの動作を制御するためのリソース

- LimitRange
- HorizontalPodAutoscaler
- PodDisruptionBudget
- CustomResourceDefinition

## 操作

- kubectl
  - kubeconfig
    - clusters...接続先クラスタ
    - users...認証情報
    - contexts...接続先と認証情報の組み合わせ
    - 例
      - ```yaml
          apiVersion: v1
          kind: Config preferences: {}
          clusters: # 接続先クラスタ
            - name: sample-cluster
              cluster:
                server: https://localhost:6443
          users: # 認証情報
            - name: sample-user
              user:
                client-certificate-data: LS0tLS1CRUdJTi...
                client-key-data: LS0tLS1CRUdJTi...
          contexts: # 接続先と認証情報の組み合わせ
            - name: sample-context
              context:
                cluster: sample-cluster
                namespace: default
                user: sample-user
          current-context: sample-context
        ```
  - マニフェスト
    - 例
      - ```yaml
          apiVersion: v1
          kind: Pod
          metadata:
            name: sample-pod
          spec:
            containers:
              - name: nginx-container
                image: nginx:1.12
        ```
    - 設計
      - 1マニフェストに1リソースといった制限はない。複数リソースを記述できる
      - 例: 「Podを起動するWorkloadsリソース」と「そのWorkloadsリソースを外部公開するDiscovery＆LBリソース」など
  - コマンド
    - kubectl apply -f \<file_name\>.yaml
