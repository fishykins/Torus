using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Unity.Entities;
using Unity.Transforms;
using Unity.Rendering;
using Unity.Mathematics;





public class Spawner : MonoBehaviour
{

    [SerializeField] private Mesh unitMesh;
    [SerializeField] private Material unitMaterial;

    // Start is called before the first frame update
    void Start()
    {
        MakeEntity();
    }

    private void MakeEntity()
    {
        EntityManager entityManager = World.DefaultGameObjectInjectionWorld.EntityManager;

        EntityArchetype archatype = entityManager.CreateArchetype(
            typeof(Translation),
            typeof(Rotation),
            typeof(RenderMesh),
            typeof(RenderBounds),
            typeof(LocalToWorld)
            );

        Entity myEntity = entityManager.CreateEntity(archatype);


        entityManager.AddComponentData(myEntity, new LocalToWorld
        {
        });


        entityManager.AddComponentData(myEntity, new Translation
        {
            Value = new float3(2f, 0f, 4f)
        });

        entityManager.AddSharedComponentData(myEntity, new RenderMesh
        {
            mesh = unitMesh,
            material  = unitMaterial
        });

        entityManager.CreateEntity();

    }
}
