//
//  Linear to Sony SLog3
//

__DEVICE__ float LinearToSlog3(float x)
{
    float result;
    if ( x >= 0.01125000f)
    {
        result = (420.0f + _log10f((x + 0.01f) / (0.18f + 0.01f)) * 261.5f) / 1023.0f;
    }
    else
    {
        result = (x * (171.2102946929f - 95.0f) / 0.01125000f + 95.0f) / 1023.0f;
    }
    return result;
}
__DEVICE__ float3 LinearToSonySLog3(float3 rgb)
{
	float3 result;

    result.x = LinearToSlog3(rgb.x);
    result.y = LinearToSlog3(rgb.y);
    result.z = LinearToSlog3(rgb.z);
    
    return result;
}
