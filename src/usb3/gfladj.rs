#[doc = "Register `GFLADJ` reader"]
pub type R = crate::R<GfladjSpec>;
#[doc = "Register `GFLADJ` writer"]
pub type W = crate::W<GfladjSpec>;
#[doc = "Field `GFLADJ_30MHZ` reader - GFLADJ_30MHZ\n\nThis field indicates the value that is used for frame length\n\nadjustment instead of considering from the sideband input signal\n\nfladj_30mhz_reg.\n\nThis enables post-silicon frame length adjustment in case the\n\ninput signal fladj_30mhz_reg is connected to a wrong value or is\n\nnot valid.\n\nFor details on how to set this value, refer to section 5.2.4,\n\n'Frame Length Adjustment Register (FLADJ),' of the xHCI\n\nSpecification."]
pub type Gfladj30mhzR = crate::FieldReader;
#[doc = "Field `GFLADJ_30MHZ` writer - GFLADJ_30MHZ\n\nThis field indicates the value that is used for frame length\n\nadjustment instead of considering from the sideband input signal\n\nfladj_30mhz_reg.\n\nThis enables post-silicon frame length adjustment in case the\n\ninput signal fladj_30mhz_reg is connected to a wrong value or is\n\nnot valid.\n\nFor details on how to set this value, refer to section 5.2.4,\n\n'Frame Length Adjustment Register (FLADJ),' of the xHCI\n\nSpecification."]
pub type Gfladj30mhzW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GFLADJ_30MHZ_SDBND_SEL` reader - GFLADJ_30MHZ_SDBND_SEL\n\nThis field selects whether to use the input signal fladj_30mhz_reg\n\nor the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the\n\nSOF/ITP. When this bit is set to:\n\n1, the controller uses the register field GFLADJ.GFLADJ_30MHZ\n\nvalue\n\n0, the controller uses the input signal fladj_30mhz_reg value"]
pub type Gfladj30mhzSdbndSelR = crate::BitReader;
#[doc = "Field `GFLADJ_30MHZ_SDBND_SEL` writer - GFLADJ_30MHZ_SDBND_SEL\n\nThis field selects whether to use the input signal fladj_30mhz_reg\n\nor the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the\n\nSOF/ITP. When this bit is set to:\n\n1, the controller uses the register field GFLADJ.GFLADJ_30MHZ\n\nvalue\n\n0, the controller uses the input signal fladj_30mhz_reg value"]
pub type Gfladj30mhzSdbndSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFLADJ_REFCLK_FLADJ` reader - GFLADJ_REFCLK_FLADJ\n\nThis field indicates the frame length adjustment to be applied\n\nwhen SOF/ITP counter is running on the ref_clk.\n\nThis register value is used to adjust the ITP interval when\n\nGCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when\n\nGLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set\n\nto 1.\n\nThe value is derived as follows:\n\nFLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)-\n\n(125000/ref_clk_period)) * ref_clk_period where:\n\n1. The ref_clk_period_integer is the integer value of the ref_clk\n\nperiod got by truncating the decimal (fractional) value that is\n\nprogrammed in the GUCTL.REF_CLK_PERIOD field.\n\n2. The ref_clk_period is the ref_clk period including the fractional\n\nvalue.\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)-\n\n(125000/41.6666))*41.6666 = 2032 (ignoring the fractional\n\nvalue)"]
pub type GfladjRefclkFladjR = crate::FieldReader<u16>;
#[doc = "Field `GFLADJ_REFCLK_FLADJ` writer - GFLADJ_REFCLK_FLADJ\n\nThis field indicates the frame length adjustment to be applied\n\nwhen SOF/ITP counter is running on the ref_clk.\n\nThis register value is used to adjust the ITP interval when\n\nGCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when\n\nGLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set\n\nto 1.\n\nThe value is derived as follows:\n\nFLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)-\n\n(125000/ref_clk_period)) * ref_clk_period where:\n\n1. The ref_clk_period_integer is the integer value of the ref_clk\n\nperiod got by truncating the decimal (fractional) value that is\n\nprogrammed in the GUCTL.REF_CLK_PERIOD field.\n\n2. The ref_clk_period is the ref_clk period including the fractional\n\nvalue.\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)-\n\n(125000/41.6666))*41.6666 = 2032 (ignoring the fractional\n\nvalue)"]
pub type GfladjRefclkFladjW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `GFLADJ_REFCLK_LPM_SEL` reader - GFLADJ_REFCLK_LPM_SEL\n\nThis bit enables the functionality of running SOF/ITP counters on\n\nthe ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit\n\nis set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1,\n\nGCTL.SOFITPSYNC must not be set to 1.\n\nWhen GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of\n\nthe suspend control of the USB 2.0 first port PHY (UTMI/ULPI)\n\nwith USB 3.0 port states is removed.\n\nNote that the ref_clk frequencies supported in this mode are\n\n16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the\n\ncore must be connected to the FREECLK of the PHY.\n\nNote: If you set this bit to 1, the\n\nGUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
pub type GfladjRefclkLpmSelR = crate::BitReader;
#[doc = "Field `GFLADJ_REFCLK_LPM_SEL` writer - GFLADJ_REFCLK_LPM_SEL\n\nThis bit enables the functionality of running SOF/ITP counters on\n\nthe ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit\n\nis set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1,\n\nGCTL.SOFITPSYNC must not be set to 1.\n\nWhen GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of\n\nthe suspend control of the USB 2.0 first port PHY (UTMI/ULPI)\n\nwith USB 3.0 port states is removed.\n\nNote that the ref_clk frequencies supported in this mode are\n\n16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the\n\ncore must be connected to the FREECLK of the PHY.\n\nNote: If you set this bit to 1, the\n\nGUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
pub type GfladjRefclkLpmSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFLADJ_REFCLK_240MHZ_DECR` reader - GFLADJ_REFCLK_240MHZ_DECR\n\nThis field indicates the decrement value that the controller applies\n\nfor each ref_clk in order to derive a frame timer in terms of a\n\n240-MHz clock.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThe value is derived as follows:\n\nGFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
pub type GfladjRefclk240mhzDecrR = crate::FieldReader;
#[doc = "Field `GFLADJ_REFCLK_240MHZ_DECR` writer - GFLADJ_REFCLK_240MHZ_DECR\n\nThis field indicates the decrement value that the controller applies\n\nfor each ref_clk in order to derive a frame timer in terms of a\n\n240-MHz clock.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThe value is derived as follows:\n\nGFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
pub type GfladjRefclk240mhzDecrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GFLADJ_REFCLK_240MHZDECR_PLS1` reader - GFLADJ_REFCLK_240MHZDECR_PLS1\n\nThis field indicates that the decrement value that the controller\n\napplies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR\n\nand GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each\n\nref_clk.\n\nSet this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1\n\nand the fractional component of 240/ref_frequency is greater\n\nthan or equal to 0.5.\n\nExamples:\n\nIf the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10\n\n3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
pub type GfladjRefclk240mhzdecrPls1R = crate::BitReader;
#[doc = "Field `GFLADJ_REFCLK_240MHZDECR_PLS1` writer - GFLADJ_REFCLK_240MHZDECR_PLS1\n\nThis field indicates that the decrement value that the controller\n\napplies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR\n\nand GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each\n\nref_clk.\n\nSet this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1\n\nand the fractional component of 240/ref_frequency is greater\n\nthan or equal to 0.5.\n\nExamples:\n\nIf the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10\n\n3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
pub type GfladjRefclk240mhzdecrPls1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - GFLADJ_30MHZ\n\nThis field indicates the value that is used for frame length\n\nadjustment instead of considering from the sideband input signal\n\nfladj_30mhz_reg.\n\nThis enables post-silicon frame length adjustment in case the\n\ninput signal fladj_30mhz_reg is connected to a wrong value or is\n\nnot valid.\n\nFor details on how to set this value, refer to section 5.2.4,\n\n'Frame Length Adjustment Register (FLADJ),' of the xHCI\n\nSpecification."]
    #[inline(always)]
    pub fn gfladj_30mhz(&self) -> Gfladj30mhzR {
        Gfladj30mhzR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - GFLADJ_30MHZ_SDBND_SEL\n\nThis field selects whether to use the input signal fladj_30mhz_reg\n\nor the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the\n\nSOF/ITP. When this bit is set to:\n\n1, the controller uses the register field GFLADJ.GFLADJ_30MHZ\n\nvalue\n\n0, the controller uses the input signal fladj_30mhz_reg value"]
    #[inline(always)]
    pub fn gfladj_30mhz_sdbnd_sel(&self) -> Gfladj30mhzSdbndSelR {
        Gfladj30mhzSdbndSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:21 - GFLADJ_REFCLK_FLADJ\n\nThis field indicates the frame length adjustment to be applied\n\nwhen SOF/ITP counter is running on the ref_clk.\n\nThis register value is used to adjust the ITP interval when\n\nGCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when\n\nGLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set\n\nto 1.\n\nThe value is derived as follows:\n\nFLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)-\n\n(125000/ref_clk_period)) * ref_clk_period where:\n\n1. The ref_clk_period_integer is the integer value of the ref_clk\n\nperiod got by truncating the decimal (fractional) value that is\n\nprogrammed in the GUCTL.REF_CLK_PERIOD field.\n\n2. The ref_clk_period is the ref_clk period including the fractional\n\nvalue.\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)-\n\n(125000/41.6666))*41.6666 = 2032 (ignoring the fractional\n\nvalue)"]
    #[inline(always)]
    pub fn gfladj_refclk_fladj(&self) -> GfladjRefclkFladjR {
        GfladjRefclkFladjR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bit 23 - GFLADJ_REFCLK_LPM_SEL\n\nThis bit enables the functionality of running SOF/ITP counters on\n\nthe ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit\n\nis set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1,\n\nGCTL.SOFITPSYNC must not be set to 1.\n\nWhen GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of\n\nthe suspend control of the USB 2.0 first port PHY (UTMI/ULPI)\n\nwith USB 3.0 port states is removed.\n\nNote that the ref_clk frequencies supported in this mode are\n\n16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the\n\ncore must be connected to the FREECLK of the PHY.\n\nNote: If you set this bit to 1, the\n\nGUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
    #[inline(always)]
    pub fn gfladj_refclk_lpm_sel(&self) -> GfladjRefclkLpmSelR {
        GfladjRefclkLpmSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - GFLADJ_REFCLK_240MHZ_DECR\n\nThis field indicates the decrement value that the controller applies\n\nfor each ref_clk in order to derive a frame timer in terms of a\n\n240-MHz clock.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThe value is derived as follows:\n\nGFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
    #[inline(always)]
    pub fn gfladj_refclk_240mhz_decr(&self) -> GfladjRefclk240mhzDecrR {
        GfladjRefclk240mhzDecrR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - GFLADJ_REFCLK_240MHZDECR_PLS1\n\nThis field indicates that the decrement value that the controller\n\napplies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR\n\nand GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each\n\nref_clk.\n\nSet this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1\n\nand the fractional component of 240/ref_frequency is greater\n\nthan or equal to 0.5.\n\nExamples:\n\nIf the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10\n\n3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
    #[inline(always)]
    pub fn gfladj_refclk_240mhzdecr_pls1(&self) -> GfladjRefclk240mhzdecrPls1R {
        GfladjRefclk240mhzdecrPls1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - GFLADJ_30MHZ\n\nThis field indicates the value that is used for frame length\n\nadjustment instead of considering from the sideband input signal\n\nfladj_30mhz_reg.\n\nThis enables post-silicon frame length adjustment in case the\n\ninput signal fladj_30mhz_reg is connected to a wrong value or is\n\nnot valid.\n\nFor details on how to set this value, refer to section 5.2.4,\n\n'Frame Length Adjustment Register (FLADJ),' of the xHCI\n\nSpecification."]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_30mhz(&mut self) -> Gfladj30mhzW<GfladjSpec> {
        Gfladj30mhzW::new(self, 0)
    }
    #[doc = "Bit 7 - GFLADJ_30MHZ_SDBND_SEL\n\nThis field selects whether to use the input signal fladj_30mhz_reg\n\nor the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the\n\nSOF/ITP. When this bit is set to:\n\n1, the controller uses the register field GFLADJ.GFLADJ_30MHZ\n\nvalue\n\n0, the controller uses the input signal fladj_30mhz_reg value"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_30mhz_sdbnd_sel(&mut self) -> Gfladj30mhzSdbndSelW<GfladjSpec> {
        Gfladj30mhzSdbndSelW::new(self, 7)
    }
    #[doc = "Bits 8:21 - GFLADJ_REFCLK_FLADJ\n\nThis field indicates the frame length adjustment to be applied\n\nwhen SOF/ITP counter is running on the ref_clk.\n\nThis register value is used to adjust the ITP interval when\n\nGCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when\n\nGLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set\n\nto 1.\n\nThe value is derived as follows:\n\nFLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)-\n\n(125000/ref_clk_period)) * ref_clk_period where:\n\n1. The ref_clk_period_integer is the integer value of the ref_clk\n\nperiod got by truncating the decimal (fractional) value that is\n\nprogrammed in the GUCTL.REF_CLK_PERIOD field.\n\n2. The ref_clk_period is the ref_clk period including the fractional\n\nvalue.\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)-\n\n(125000/41.6666))*41.6666 = 2032 (ignoring the fractional\n\nvalue)"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_fladj(&mut self) -> GfladjRefclkFladjW<GfladjSpec> {
        GfladjRefclkFladjW::new(self, 8)
    }
    #[doc = "Bit 23 - GFLADJ_REFCLK_LPM_SEL\n\nThis bit enables the functionality of running SOF/ITP counters on\n\nthe ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit\n\nis set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1,\n\nGCTL.SOFITPSYNC must not be set to 1.\n\nWhen GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of\n\nthe suspend control of the USB 2.0 first port PHY (UTMI/ULPI)\n\nwith USB 3.0 port states is removed.\n\nNote that the ref_clk frequencies supported in this mode are\n\n16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the\n\ncore must be connected to the FREECLK of the PHY.\n\nNote: If you set this bit to 1, the\n\nGUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_lpm_sel(&mut self) -> GfladjRefclkLpmSelW<GfladjSpec> {
        GfladjRefclkLpmSelW::new(self, 23)
    }
    #[doc = "Bits 24:30 - GFLADJ_REFCLK_240MHZ_DECR\n\nThis field indicates the decrement value that the controller applies\n\nfor each ref_clk in order to derive a frame timer in terms of a\n\n240-MHz clock.\n\nThis field must be programmed to a non-zero value only if\n\nGFLADJ_REFCLK_LPM_SEL is set to 1.\n\nThe value is derived as follows:\n\nGFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency\n\nExamples: If the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_240mhz_decr(&mut self) -> GfladjRefclk240mhzDecrW<GfladjSpec> {
        GfladjRefclk240mhzDecrW::new(self, 24)
    }
    #[doc = "Bit 31 - GFLADJ_REFCLK_240MHZDECR_PLS1\n\nThis field indicates that the decrement value that the controller\n\napplies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR\n\nand GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each\n\nref_clk.\n\nSet this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1\n\nand the fractional component of 240/ref_frequency is greater\n\nthan or equal to 0.5.\n\nExamples:\n\nIf the ref_clk is 24 MHz then\n\n1. GUCTL.REF_CLK_PERIOD = 41\n\n2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10\n\n3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_240mhzdecr_pls1(&mut self) -> GfladjRefclk240mhzdecrPls1W<GfladjSpec> {
        GfladjRefclk240mhzdecrPls1W::new(self, 31)
    }
}
#[doc = "Global Frame Length Adjustment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfladj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfladj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfladjSpec;
impl crate::RegisterSpec for GfladjSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfladj::R`](R) reader structure"]
impl crate::Readable for GfladjSpec {}
#[doc = "`write(|w| ..)` method takes [`gfladj::W`](W) writer structure"]
impl crate::Writable for GfladjSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GFLADJ to value 0"]
impl crate::Resettable for GfladjSpec {
    const RESET_VALUE: u32 = 0;
}
