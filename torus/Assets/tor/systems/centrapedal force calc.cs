using Unity.Entities;
using Unity.Transforms;
using Unity.Mathematics;
using System;

public class NewBehaviourScript : ComponentSystem
{
    protected override void OnUpdate()
    {
        Entities.ForEach((ref Translation trans, ref Centrapedalforce centrapedalforce) =>
        {
            float xPosition = trans.Value.x;
            float zPosition = trans.Value.z;


            double dist = math.sqrt((xPosition*xPosition) + (zPosition* zPosition));
            float distSingle = (float)dist;
            float acceleration = math.pow(1.00125f, distSingle);
            centrapedalforce.value = acceleration;

        });
    }
}
