#[doc = "Register `GDBGLSPMUX` reader"]
pub type R = crate::R<GdbglspmuxSpec>;
#[doc = "Register `GDBGLSPMUX` writer"]
pub type W = crate::W<GdbglspmuxSpec>;
#[doc = "Field `EPSELECT` reader - Device Endpoint Select\n\nSelects the Endpoint debug information presented in the\n\nGDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of\n\nHOSTSELECT, Selects the LSP debug information presented in the\n\nGDBGLSP register in host mode."]
pub type EpselectR = crate::FieldReader;
#[doc = "Field `EPSELECT` writer - Device Endpoint Select\n\nSelects the Endpoint debug information presented in the\n\nGDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of\n\nHOSTSELECT, Selects the LSP debug information presented in the\n\nGDBGLSP register in host mode."]
pub type EpselectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEVSELECT` reader - Device LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the\n\nLSP debug information presented in the GDBGLSP register in host\n\nmode."]
pub type DevselectR = crate::FieldReader;
#[doc = "Field `DEVSELECT` writer - Device LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the\n\nLSP debug information presented in the GDBGLSP register in host\n\nmode."]
pub type DevselectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOSTSELECT` reader - Host LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in host mode."]
pub type HostselectR = crate::FieldReader;
#[doc = "Field `HOSTSELECT` writer - Host LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in host mode."]
pub type HostselectW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ENDBC` reader - EnDbc\n\nEnable debugging of Debug capablity LSP in Host mode. Use\n\nHostSelect to select DbC LSP debug information presented in the\n\nGDBGLSP register."]
pub type EndbcR = crate::BitReader;
#[doc = "Field `ENDBC` writer - EnDbc\n\nEnable debugging of Debug capablity LSP in Host mode. Use\n\nHostSelect to select DbC LSP debug information presented in the\n\nGDBGLSP register."]
pub type EndbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOGIC_ANALYZER_TRACE` reader - Logic Analyzer Trace Port MUX Select\n\nCurrently only bits\\[21:16\\]
are used. A value of 6'h3F drives '0's\n\non the logic_analyzer_trace signal. If you plan to OR (instead\n\nusing a mux) this signal with other trace signals in your system\n\nto generate a common trace signal, you can use this feature."]
pub type LogicAnalyzerTraceR = crate::FieldReader;
#[doc = "Field `LOGIC_ANALYZER_TRACE` writer - Logic Analyzer Trace Port MUX Select\n\nCurrently only bits\\[21:16\\]
are used. A value of 6'h3F drives '0's\n\non the logic_analyzer_trace signal. If you plan to OR (instead\n\nusing a mux) this signal with other trace signals in your system\n\nto generate a common trace signal, you can use this feature."]
pub type LogicAnalyzerTraceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Device Endpoint Select\n\nSelects the Endpoint debug information presented in the\n\nGDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of\n\nHOSTSELECT, Selects the LSP debug information presented in the\n\nGDBGLSP register in host mode."]
    #[inline(always)]
    pub fn epselect(&self) -> EpselectR {
        EpselectR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Device LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the\n\nLSP debug information presented in the GDBGLSP register in host\n\nmode."]
    #[inline(always)]
    pub fn devselect(&self) -> DevselectR {
        DevselectR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Host LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in host mode."]
    #[inline(always)]
    pub fn hostselect(&self) -> HostselectR {
        HostselectR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - EnDbc\n\nEnable debugging of Debug capablity LSP in Host mode. Use\n\nHostSelect to select DbC LSP debug information presented in the\n\nGDBGLSP register."]
    #[inline(always)]
    pub fn endbc(&self) -> EndbcR {
        EndbcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Logic Analyzer Trace Port MUX Select\n\nCurrently only bits\\[21:16\\]
are used. A value of 6'h3F drives '0's\n\non the logic_analyzer_trace signal. If you plan to OR (instead\n\nusing a mux) this signal with other trace signals in your system\n\nto generate a common trace signal, you can use this feature."]
    #[inline(always)]
    pub fn logic_analyzer_trace(&self) -> LogicAnalyzerTraceR {
        LogicAnalyzerTraceR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Device Endpoint Select\n\nSelects the Endpoint debug information presented in the\n\nGDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of\n\nHOSTSELECT, Selects the LSP debug information presented in the\n\nGDBGLSP register in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn epselect(&mut self) -> EpselectW<GdbglspmuxSpec> {
        EpselectW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Device LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the\n\nLSP debug information presented in the GDBGLSP register in host\n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn devselect(&mut self) -> DevselectW<GdbglspmuxSpec> {
        DevselectW::new(self, 4)
    }
    #[doc = "Bits 8:13 - Host LSP Select\n\nSelects the LSP debug information presented in the GDBGLSP\n\nregister in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn hostselect(&mut self) -> HostselectW<GdbglspmuxSpec> {
        HostselectW::new(self, 8)
    }
    #[doc = "Bit 15 - EnDbc\n\nEnable debugging of Debug capablity LSP in Host mode. Use\n\nHostSelect to select DbC LSP debug information presented in the\n\nGDBGLSP register."]
    #[inline(always)]
    #[must_use]
    pub fn endbc(&mut self) -> EndbcW<GdbglspmuxSpec> {
        EndbcW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Logic Analyzer Trace Port MUX Select\n\nCurrently only bits\\[21:16\\]
are used. A value of 6'h3F drives '0's\n\non the logic_analyzer_trace signal. If you plan to OR (instead\n\nusing a mux) this signal with other trace signals in your system\n\nto generate a common trace signal, you can use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn logic_analyzer_trace(&mut self) -> LogicAnalyzerTraceW<GdbglspmuxSpec> {
        LogicAnalyzerTraceW::new(self, 16)
    }
}
#[doc = "Global Debug LSP MUX Register - Device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbglspmux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdbglspmux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdbglspmuxSpec;
impl crate::RegisterSpec for GdbglspmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdbglspmux::R`](R) reader structure"]
impl crate::Readable for GdbglspmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gdbglspmux::W`](W) writer structure"]
impl crate::Writable for GdbglspmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDBGLSPMUX to value 0x003f_0000"]
impl crate::Resettable for GdbglspmuxSpec {
    const RESET_VALUE: u32 = 0x003f_0000;
}
