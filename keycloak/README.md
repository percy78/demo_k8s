# How to render locally
Use following command in tempo distributed dev overlay directory:
```
$ kubectl kustomize --enable-helm --load-restrictor LoadRestrictionsNone > _rendered.yaml
```
