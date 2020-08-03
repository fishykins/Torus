using Unity.Entities;
using Unity.Transforms;
using Unity.Mathematics;
using System;

public class ReverseGravity : ComponentSystem
{
    protected override void OnUpdate()
    {
        Entities.ForEach((ref Translation trans, ref Centrapedalforce centrapedalforce) =>
        {
            float sin = math.sin(centrapedalforce.value);
            float cos = math.cos(centrapedalforce.value);
        });

    }
}