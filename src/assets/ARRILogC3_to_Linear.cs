//
//  ARRI LogC3 to Linear
//

__DEVICE__ float LogC3ToLinear(float p_X)
{
	const float midGraySignal = 0.01;
	const float cut = 1.0 / 9.0;
	const float slope = 3.9086503371;
	const float offset =  -1.3885369913;
	const float encOffset = 0.3855369987;
	const float gain = 800.0 / 400.0;
	const float encGain = 0.2471896383;
	const float gray = 0.005;
	const float nz = 0.0522722750;

    float result = (p_X - encOffset) / encGain;
    float ns = (result - offset) / slope;
    if (ns > cut)
    {
        ns = _powf(10.0f, result);
    }
    ns = (ns - nz) * gray;
    return ns * (0.18f * gain / midGraySignal);
}
__DEVICE__ float3 ArriLogC3ToLinear(float3 rgb)
{
	float3 result;

    result.x = LogC3ToLinear(rgb.x);
    result.y = LogC3ToLinear(rgb.y);
    result.z = LogC3ToLinear(rgb.z);
    
    return result;
}

