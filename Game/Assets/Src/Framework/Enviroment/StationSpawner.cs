using UnityEngine;
using Unity.Entities;
using Unity.Transforms;
using Unity.Rendering;
using Unity.Mathematics;

public class StationSpawner : MonoBehaviour
{
    [SerializeField] private Mesh moduleMesh;// these are enterable things
    [SerializeField] private Material moduleMat;


    // Start is called before the first frame update
    void Start()
    {
        MakeEntity();
    }

    private void MakeEntity()
    {
        EntityManager entityManager = World.DefaultGameObjectInjectionWorld.EntityManager;
        

        for (int i = 0; i < 18; i++)
        {
            EntityArchetype archatype = entityManager.CreateArchetype(
                typeof(Translation),
                typeof(Rotation),
                typeof(RenderMesh),
                typeof(RenderBounds),
                typeof(LocalToWorld)
            );


            Entity module = entityManager.CreateEntity(archatype);

            entityManager.SetComponentData(module, new Translation
            {
                Value = new float3(0f, 0f, 1f)
            });

            entityManager.SetComponentData(module, new RenderBounds
            {
                Value = moduleMesh.bounds.ToAABB()
            });

            entityManager.SetComponentData(module, new Rotation
            {
                Value = quaternion.AxisAngle(new float3(1f, 0f, 0f), 2 * math.PI / 18 * i)
            });

            entityManager.SetSharedComponentData<RenderMesh>(module,new RenderMesh { mesh = moduleMesh , material = moduleMat } );
        }
    }
}
