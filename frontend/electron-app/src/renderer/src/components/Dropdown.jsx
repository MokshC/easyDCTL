function ColorSpaceDropdown({ io, onChange }) {
    
    if (io === "input") {
        return (
            <select name="input-color-space" id="input-color-space" required className="dropdown" onChange={onChange}>
                <option value="-1">Use timeline</option>
                <option value="0">ACES APO</option>
                <option value="1">ARRI Wide Gamut 3</option>
                <option value="2">RED Wide Gamut RGB</option>
                <option value="3">SONY S-Gamut3.Cine</option>
                <option value="4">No Change (only for input)</option>
            </select>
        )
    } else {
        return (
            <select name="output-color-space" id="output-color-space" required className="dropdown">
                <option value="-1">Use timeline</option>
                <option value="0">ACES APO</option>
                <option value="1">ARRI Wide Gamut 3</option>
                <option value="2">RED Wide Gamut RGB</option>
                <option value="3">SONY S-Gamut3.Cine</option>
                <option value="4">No Change (only for input)</option>
            </select>
        )
    }
}

function GammaDropdown({ io, onChange }) {

    if (io === "input") {
        return (
            <select name="input-gamma" id="input-gamma" required className="dropdown" onChange={onChange}>
                <option value="-1">Use timeline</option>
                <option value="0">Linear</option>
                <option value="1">ARRI LogC3</option>
                <option value="2">RED Log3G10</option>
                <option value="3">SONY S-log3</option>
                <option value="4">No Change (only for input)</option>
            </select>
        )
    } else {
        return (
            <select name="output-gamma" id="output-gamma" required className="dropdown">
                <option value="-1">Use timeline</option>
                <option value="0">Linear</option>
                <option value="1">ARRI LogC3</option>
                <option value="2">RED Log3G10</option>
                <option value="3">SONY S-log3</option>
                <option value="4">No Change (only for input)</option>
            </select>
        )
    }
}

export { ColorSpaceDropdown, GammaDropdown }