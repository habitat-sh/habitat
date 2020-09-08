+++
title = "Azure Container Services (AKS)"
description = "Azure and Kubernetes K8"

[menu]
  [menu.habitat]
    title = "Azure Container Services (AKS)"
    identifier = "habitat/containers/aks-and-habitat.md Habitat Azure Kubernetes"
    parent = "habitat/containers"
    weight = 20

+++

[Azure Container Services (AKS)](https://azure.microsoft.com/services/container-service/)
is a fully managed Kubernetes service running on the Azure platform.

## Azure Container Registry (ACR)

Azure Container Registry is a managed Docker container registry service used for storing private Docker container images. It's a fully managed Azure resource and gives you local, network-close storage of your container images when deploying to AKS. Chef Habitat Builder has native integration with this service so you can publish your packages directly to Azure Container Registry.

In order to do this you need to create an Azure Service Principal that has `Owner` rights
on your ACR instance. You can do this with the following script, changing the environment
variable values to match your environment.

```
  !/bin/bash

    R_RESOURCE_GROUP=myACRResourceGroup
    R_NAME=myACRRegistry
BLDR_PRINCIPAL_NAME=myPrincipalName
BLDR_PRINCIPAL_PASSWORD="ThisIsVeryStrongPassword"

    Create Service Principal for Chef Habitat Builder
    R_ID=$(az acr show --name $ACR_NAME --resource-group $ACR_RESOURCE_GROUP --query "id" --output tsv)
     ad sp create-for-rbac --scopes $ACR_ID --role Owner --password "$BLDR_PRINCIPAL_PASSWORD" --name $BLDR_PRINCIPAL_NAME
BLDR_ID=$(az ad sp list --display-name $BLDR_PRINCIPAL_NAME  --query "[].appId" --output tsv)

    ho "Configuration details for Habitat Builder Principal:"
echo "  ID : $BLDR_ID"
echo "  Password : $BLDR_PRINCIPAL_PASSWORD"
```

Note: The unique Service Principal Name (the UUID) should be provided in the Chef Habitat Builder
configuration.

## Connecting ACR and AKS for Chef Habitat Operator

Since ACR is a private Docker registry, AKS must be authorized to pull images from it. The best way is to
create a role assignment on the Service Principal that is automatically created for AKS, granting it
`Reader` access on your ACR instance.

To do this you can use the following script, changing the environment variable values to match your configuration.

```
#!/bin/bash

AKS_RESOURCE_GROUP=myAKSResourceGroup
AKS_CLUSTER_NAME=myAKSCluster
ACR_RESOURCE_GROUP=myACRResourceGroup
ACR_NAME=myACRRegistry

# Get the id of the service principal configured for AKS
CLIENT_ID=$(az aks show --resource-group $AKS_RESOURCE_GROUP --name $AKS_CLUSTER_NAME --query "servicePrincipalProfile.clientId" --output tsv)

# Get the ACR Registry Resource ID
ACR_ID=$(az acr show --name $ACR_NAME --resource-group $ACR_RESOURCE_GROUP --query "id" --output tsv)

# Create Role Assignment
az role assignment create --assignee $CLIENT_ID --role Reader --scope $ACR_ID
```

## Related Reading

* [Authenticate with Azure Container Registry from Azure Container Service](https://docs.microsoft.com/azure/container-registry/container-registry-auth-aks#grant-aks-access-to-acr)
