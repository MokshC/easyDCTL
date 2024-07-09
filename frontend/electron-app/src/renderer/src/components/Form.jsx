import {ColorSpaceDropdown, GammaDropdown} from "./Dropdown"
import '../assets/index.css'
import { useState } from "react"

function Form() {
    const [dctlData, setDctlData] = useState('');

    const handleColorSpace = (e) => {
        const outputColorSpace = document.getElementById('output-color-space');

        if (e.target.value === "4") {
            outputColorSpace.setAttribute("disabled", true);
        } else {
            outputColorSpace.removeAttribute("disabled");
        }
    }

    const handleGamma = (e) => {
        const outputGamma = document.getElementById('output-gamma');

        if (e.target.value === "4") {
            outputGamma.setAttribute("disabled", true);
        } else {
            outputGamma.removeAttribute("disabled");
        }
    }

    const handleSwap = (e) => {
        e.preventDefault();

        const inputColorSpace = document.getElementById('input-color-space');
        const outputColorSpace = document.getElementById('output-color-space');
        const inputGamma = document.getElementById('input-gamma');
        const outputGamma = document.getElementById('output-gamma');

        const initialInputColorSpace = inputColorSpace.value;
        const initialOutputColorSpace = outputColorSpace.value;
        const initialInputGamma = inputGamma.value;
        const initialOutputGamma = outputGamma.value;

        if (inputColorSpace.value != '4') { 
            inputColorSpace.value = initialOutputColorSpace;
            outputColorSpace.value = initialInputColorSpace;
        };

        if (inputGamma.value != '4') { 
            inputGamma.value = initialOutputGamma;
            outputGamma.value = initialInputGamma; 
        };
    }

    const generateRandomDCTL = () => {
        const header = `__DEVICE__ void main(
          float4 outColor : SV_Target,
          float2 inCoord : VPOS)
        {
          float4 inColor = tex2D(Color, inCoord);
        `;
        const body = `  outColor = float4(
          inColor.r * ${Math.random().toFixed(2)},
          inColor.g * ${Math.random().toFixed(2)},
          inColor.b * ${Math.random().toFixed(2)},
          1.0);
        }`;
    
        const randomDCTL = header + body;
        setDctlData(randomDCTL);
      };

    const handleExport = async (e) => {
        e.preventDefault();

        generateRandomDCTL();

        try {
            const result = await window.electronAPI.saveDCTLFile(dctlData);
            if (result.success) {
              alert('File saved successfully!');
            } else {
              alert('File save canceled.');
            }
          } catch (error) {
            console.error('Error exporting file:', error);
          }
    }

    return (
        <form action="" method="POST" className="form">
            <section className="dropdown-section">
                <label htmlFor="input-color-space" className="label">Input Color Space</label>
                <ColorSpaceDropdown io="input" onChange={handleColorSpace}/>
                <label htmlFor="input-gamma" className="label">Input Gamma</label>
                <GammaDropdown io="input" onChange={handleGamma}/>
                <label htmlFor="output-color-space" className="label">Output Color Space</label>
                <ColorSpaceDropdown io="output" />
                <label htmlFor="output-gamma" className="label">Output Gamma</label>
                <GammaDropdown io="output" />
            </section>
            <section className="button-section">
                <button onClick={handleSwap}>Swap</button>
                <button onClick={handleExport}>Export</button>
            </section>

        </form>
    )
}

export default Form