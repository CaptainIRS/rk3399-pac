#[doc = "Register `USB3_GDBGLSPMUX` reader"]
pub type R = crate::R<Usb3GdbglspmuxSpec>;
#[doc = "Register `USB3_GDBGLSPMUX` writer"]
pub type W = crate::W<Usb3GdbglspmuxSpec>;
#[doc = "Field `EPSELECT` reader - Device Endpoint Select Selects the Endpoint debug information presented in the GDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
pub type EpselectR = crate::FieldReader;
#[doc = "Field `EPSELECT` writer - Device Endpoint Select Selects the Endpoint debug information presented in the GDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
pub type EpselectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEVSELECT` reader - Device LSP Select Selects the LSP debug information presented in the GDBGLSP register in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
pub type DevselectR = crate::FieldReader;
#[doc = "Field `DEVSELECT` writer - Device LSP Select Selects the LSP debug information presented in the GDBGLSP register in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
pub type DevselectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOSTSELECT` reader - Host LSP Select Selects the LSP debug information presented in the GDBGLSP register in host mode."]
pub type HostselectR = crate::FieldReader;
#[doc = "Field `HOSTSELECT` writer - Host LSP Select Selects the LSP debug information presented in the GDBGLSP register in host mode."]
pub type HostselectW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ENDBC` reader - EnDbc Enable debugging of Debug capablity LSP in Host mode. Use HostSelect to select DbC LSP debug information presented in the GDBGLSP register."]
pub type EndbcR = crate::BitReader;
#[doc = "Field `ENDBC` writer - EnDbc Enable debugging of Debug capablity LSP in Host mode. Use HostSelect to select DbC LSP debug information presented in the GDBGLSP register."]
pub type EndbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOGIC_ANALYZER_TRACE` reader - Logic Analyzer Trace Port MUX Select Currently only bits\\[21:16\\]
are used. A value of 6'h3F drives \"0\"s on the logic_analyzer_trace signal. If you plan to OR (instead using a mux) this signal with other trace signals in your system to generate a common trace signal, you can use this feature."]
pub type LogicAnalyzerTraceR = crate::FieldReader;
#[doc = "Field `LOGIC_ANALYZER_TRACE` writer - Logic Analyzer Trace Port MUX Select Currently only bits\\[21:16\\]
are used. A value of 6'h3F drives \"0\"s on the logic_analyzer_trace signal. If you plan to OR (instead using a mux) this signal with other trace signals in your system to generate a common trace signal, you can use this feature."]
pub type LogicAnalyzerTraceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Device Endpoint Select Selects the Endpoint debug information presented in the GDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
    #[inline(always)]
    pub fn epselect(&self) -> EpselectR {
        EpselectR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Device LSP Select Selects the LSP debug information presented in the GDBGLSP register in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
    #[inline(always)]
    pub fn devselect(&self) -> DevselectR {
        DevselectR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Host LSP Select Selects the LSP debug information presented in the GDBGLSP register in host mode."]
    #[inline(always)]
    pub fn hostselect(&self) -> HostselectR {
        HostselectR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - EnDbc Enable debugging of Debug capablity LSP in Host mode. Use HostSelect to select DbC LSP debug information presented in the GDBGLSP register."]
    #[inline(always)]
    pub fn endbc(&self) -> EndbcR {
        EndbcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Logic Analyzer Trace Port MUX Select Currently only bits\\[21:16\\]
are used. A value of 6'h3F drives \"0\"s on the logic_analyzer_trace signal. If you plan to OR (instead using a mux) this signal with other trace signals in your system to generate a common trace signal, you can use this feature."]
    #[inline(always)]
    pub fn logic_analyzer_trace(&self) -> LogicAnalyzerTraceR {
        LogicAnalyzerTraceR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Device Endpoint Select Selects the Endpoint debug information presented in the GDBGEPINFO registers in device mode. Or bit\\[3:0\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn epselect(&mut self) -> EpselectW<Usb3GdbglspmuxSpec> {
        EpselectW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Device LSP Select Selects the LSP debug information presented in the GDBGLSP register in device mode. Or bit\\[7:4\\]
of HOSTSELECT, Selects the LSP debug information presented in the GDBGLSP register in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn devselect(&mut self) -> DevselectW<Usb3GdbglspmuxSpec> {
        DevselectW::new(self, 4)
    }
    #[doc = "Bits 8:13 - Host LSP Select Selects the LSP debug information presented in the GDBGLSP register in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn hostselect(&mut self) -> HostselectW<Usb3GdbglspmuxSpec> {
        HostselectW::new(self, 8)
    }
    #[doc = "Bit 15 - EnDbc Enable debugging of Debug capablity LSP in Host mode. Use HostSelect to select DbC LSP debug information presented in the GDBGLSP register."]
    #[inline(always)]
    #[must_use]
    pub fn endbc(&mut self) -> EndbcW<Usb3GdbglspmuxSpec> {
        EndbcW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Logic Analyzer Trace Port MUX Select Currently only bits\\[21:16\\]
are used. A value of 6'h3F drives \"0\"s on the logic_analyzer_trace signal. If you plan to OR (instead using a mux) this signal with other trace signals in your system to generate a common trace signal, you can use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn logic_analyzer_trace(&mut self) -> LogicAnalyzerTraceW<Usb3GdbglspmuxSpec> {
        LogicAnalyzerTraceW::new(self, 16)
    }
}
#[doc = "Global Debug LSP MUX Register - Device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbglspmux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdbglspmux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GdbglspmuxSpec;
impl crate::RegisterSpec for Usb3GdbglspmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gdbglspmux::R`](R) reader structure"]
impl crate::Readable for Usb3GdbglspmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gdbglspmux::W`](W) writer structure"]
impl crate::Writable for Usb3GdbglspmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GDBGLSPMUX to value 0x003f_0000"]
impl crate::Resettable for Usb3GdbglspmuxSpec {
    const RESET_VALUE: u32 = 0x003f_0000;
}
