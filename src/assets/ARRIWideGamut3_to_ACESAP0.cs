//
//  ARRI WG 3 to ACES AP0
//

__DEVICE__ float3 ArriWG3ToAcesAp0(float3 rgb)
{
    float3 result;
    
	result.x = rgb.x * 6.8020600000000e-01 + rgb.y * 2.3613700000000e-01 + rgb.z * 8.3658000000000e-02;
	result.y = rgb.x * 8.5415000000000e-02 + rgb.y * 1.0174710000000e+00 + rgb.z * -1.0288600000000e-01;
	result.z = rgb.x * 2.0570000000000e-03 + rgb.y * -6.2563000000000e-02 + rgb.z * 1.0605060000000e+00;
    
    return result;
}


