#[doc = "Register `DDR_PI_REG_58` reader"]
pub type R = crate::R<DdrPiReg58Spec>;
#[doc = "Register `DDR_PI_REG_58` writer"]
pub type W = crate::W<DdrPiReg58Spec>;
#[doc = "Field `PI_DFI40_POLARITY` reader - Control polarity of dfi_wrdata_cs_n/dfi_rddata_cs_n generated by\n\nPI. It need match with that of controller's polarity. If controller's\n\ndfi_wrdata_cs_n/ dfi_rddata_cs_n is high active, pi_dfi40_polarity\n\nshould be 1, otherwise, it should be 0. If LPDDR4 connected, it is\n\nrecommended to set to 1 to match with latest DFI 4.0 protocol."]
pub type PiDfi40PolarityR = crate::BitReader;
#[doc = "Field `PI_DFI40_POLARITY` writer - Control polarity of dfi_wrdata_cs_n/dfi_rddata_cs_n generated by\n\nPI. It need match with that of controller's polarity. If controller's\n\ndfi_wrdata_cs_n/ dfi_rddata_cs_n is high active, pi_dfi40_polarity\n\nshould be 1, otherwise, it should be 0. If LPDDR4 connected, it is\n\nrecommended to set to 1 to match with latest DFI 4.0 protocol."]
pub type PiDfi40PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enables 16/32bit DRAM configuration.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pi16bitDramConnect {
    #[doc = "1: 16-bit DRAM"]
    B1 = 1,
    #[doc = "0: 32-bit DRAM"]
    B0 = 0,
}
impl From<Pi16bitDramConnect> for bool {
    #[inline(always)]
    fn from(variant: Pi16bitDramConnect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI_16BIT_DRAM_CONNECT` reader - Enables 16/32bit DRAM configuration."]
pub type Pi16bitDramConnectR = crate::BitReader<Pi16bitDramConnect>;
impl Pi16bitDramConnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pi16bitDramConnect {
        match self.bits {
            true => Pi16bitDramConnect::B1,
            false => Pi16bitDramConnect::B0,
        }
    }
    #[doc = "16-bit DRAM"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pi16bitDramConnect::B1
    }
    #[doc = "32-bit DRAM"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pi16bitDramConnect::B0
    }
}
#[doc = "Field `PI_16BIT_DRAM_CONNECT` writer - Enables 16/32bit DRAM configuration."]
pub type Pi16bitDramConnectW<'a, REG> = crate::BitWriter<'a, REG, Pi16bitDramConnect>;
impl<'a, REG> Pi16bitDramConnectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit DRAM"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pi16bitDramConnect::B1)
    }
    #[doc = "32-bit DRAM"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pi16bitDramConnect::B0)
    }
}
#[doc = "Field `PI_TDFI_CTRL_DELAY_F0` reader - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f0' of the parameter name is omitted when in non-DFS\n\nmode."]
pub type PiTdfiCtrlDelayF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F0` writer - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f0' of the parameter name is omitted when in non-DFS\n\nmode."]
pub type PiTdfiCtrlDelayF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F1` reader - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f1' of the parameter name is omitted when in non-DFS\n\nmode."]
pub type PiTdfiCtrlDelayF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F1` writer - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f1' of the parameter name is omitted when in non-DFS\n\nmode."]
pub type PiTdfiCtrlDelayF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Control polarity of dfi_wrdata_cs_n/dfi_rddata_cs_n generated by\n\nPI. It need match with that of controller's polarity. If controller's\n\ndfi_wrdata_cs_n/ dfi_rddata_cs_n is high active, pi_dfi40_polarity\n\nshould be 1, otherwise, it should be 0. If LPDDR4 connected, it is\n\nrecommended to set to 1 to match with latest DFI 4.0 protocol."]
    #[inline(always)]
    pub fn pi_dfi40_polarity(&self) -> PiDfi40PolarityR {
        PiDfi40PolarityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables 16/32bit DRAM configuration."]
    #[inline(always)]
    pub fn pi_16bit_dram_connect(&self) -> Pi16bitDramConnectR {
        Pi16bitDramConnectR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f0' of the parameter name is omitted when in non-DFS\n\nmode."]
    #[inline(always)]
    pub fn pi_tdfi_ctrl_delay_f0(&self) -> PiTdfiCtrlDelayF0R {
        PiTdfiCtrlDelayF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f1' of the parameter name is omitted when in non-DFS\n\nmode."]
    #[inline(always)]
    pub fn pi_tdfi_ctrl_delay_f1(&self) -> PiTdfiCtrlDelayF1R {
        PiTdfiCtrlDelayF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Control polarity of dfi_wrdata_cs_n/dfi_rddata_cs_n generated by\n\nPI. It need match with that of controller's polarity. If controller's\n\ndfi_wrdata_cs_n/ dfi_rddata_cs_n is high active, pi_dfi40_polarity\n\nshould be 1, otherwise, it should be 0. If LPDDR4 connected, it is\n\nrecommended to set to 1 to match with latest DFI 4.0 protocol."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfi40_polarity(&mut self) -> PiDfi40PolarityW<DdrPiReg58Spec> {
        PiDfi40PolarityW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables 16/32bit DRAM configuration."]
    #[inline(always)]
    #[must_use]
    pub fn pi_16bit_dram_connect(&mut self) -> Pi16bitDramConnectW<DdrPiReg58Spec> {
        Pi16bitDramConnectW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f0' of the parameter name is omitted when in non-DFS\n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrl_delay_f0(&mut self) -> PiTdfiCtrlDelayF0W<DdrPiReg58Spec> {
        PiTdfiCtrlDelayF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the\n\ndelay between a DFI command change and a memory command.\n\nThe suffix '_f1' of the parameter name is omitted when in non-DFS\n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrl_delay_f1(&mut self) -> PiTdfiCtrlDelayF1W<DdrPiReg58Spec> {
        PiTdfiCtrlDelayF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg58Spec;
impl crate::RegisterSpec for DdrPiReg58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_58::R`](R) reader structure"]
impl crate::Readable for DdrPiReg58Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_58::W`](W) writer structure"]
impl crate::Writable for DdrPiReg58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_58 to value 0x0100"]
impl crate::Resettable for DdrPiReg58Spec {
    const RESET_VALUE: u32 = 0x0100;
}
