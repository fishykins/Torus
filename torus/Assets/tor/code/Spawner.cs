using UnityEngine;
using Unity.Entities;
using Unity.Transforms;
using Unity.Rendering;
using Unity.Mathematics;





public class Spawner : MonoBehaviour
{

    [SerializeField] private Mesh unitMesh;// these are enterable things
    [SerializeField] private Material unitMaterial;
    [SerializeField] private GameObject gameObjectPrefab;

    private Entity entityPrefab;
    private World defaltWorld;
    private EntityManager entityManager;

    // Start is called before the first frame update
    void Start()
    {
        defaltWorld = World.DefaultGameObjectInjectionWorld;
        entityManager = defaltWorld.EntityManager;

        GameObjectConversionSettings settings = GameObjectConversionSettings.FromWorld(defaltWorld,null);
        entityPrefab = GameObjectConversionUtility.ConvertGameObjectHierarchy(gameObjectPrefab, settings);

        //InstantiateEntity(new float3(4f,0f, 4f));//make one entity

        //InstantiateEntityGrid(10, 10, 1f);//greate a grid of entities
    }

    private void InstantiateEntity(float3 positon)//take the position vector
    {
        Entity myEntity = entityManager.Instantiate(entityPrefab);//create the entity
        entityManager.SetComponentData(myEntity, new Translation//add data to it
        {
            Value = positon
        });
    } 

    private void InstantiateEntityGrid(int dimx, int dimy, float spacing = 1f)
    {
        for (int i = 0; i< dimx; i++)//create x values
        {
            for (int j = 0; j < dimy; j++)//create y values
            {
                InstantiateEntity(new float3(i*spacing, j*spacing, 4f));
            }
        }
                     
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

        //entityManager.CreateEntity(archatype);
        //entityManager.AddComponentData(myEntity, new LocalToWorld
        //{
        // });


        entityManager.AddComponentData(myEntity, new Translation
        {
            Value = new float3(0f, 0f, 1f)
        });

       //ntityManager.AddComponentData(myEntity, new Rotation
       //
       //   Value = quaternion.EulerXYZ(new float3(0f, 45f, 0f))
       //);

        entityManager.AddSharedComponentData(myEntity, new RenderMesh
        {
            mesh = unitMesh,
            material  = unitMaterial
        });

        

    }
}
