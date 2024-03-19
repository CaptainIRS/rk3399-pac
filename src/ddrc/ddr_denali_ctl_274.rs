#[doc = "Register `DDR_DENALI_CTL_274` reader"]
pub type R = crate::R<DdrDenaliCtl274Spec>;
#[doc = "Register `DDR_DENALI_CTL_274` writer"]
pub type W = crate::W<DdrDenaliCtl274Spec>;
#[doc = "Field `MEM_RST_VALID` reader - Register access to mem_rst_valid signal."]
pub type MemRstValidR = crate::BitReader;
#[doc = "Field `DLL_RST_DELAY` reader - Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstDelayR = crate::FieldReader<u16>;
#[doc = "Field `DLL_RST_DELAY` writer - Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DLL_RST_ADJ_DLY` reader - Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstAdjDlyR = crate::FieldReader;
#[doc = "Field `DLL_RST_ADJ_DLY` writer - Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstAdjDlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Register access to mem_rst_valid signal."]
    #[inline(always)]
    pub fn mem_rst_valid(&self) -> MemRstValidR {
        MemRstValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    pub fn dll_rst_delay(&self) -> DllRstDelayR {
        DllRstDelayR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    pub fn dll_rst_adj_dly(&self) -> DllRstAdjDlyR {
        DllRstAdjDlyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:23 - Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn dll_rst_delay(&mut self) -> DllRstDelayW<DdrDenaliCtl274Spec> {
        DllRstDelayW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn dll_rst_adj_dly(&mut self) -> DllRstAdjDlyW<DdrDenaliCtl274Spec> {
        DllRstAdjDlyW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_274::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_274::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl274Spec;
impl crate::RegisterSpec for DdrDenaliCtl274Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_274::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl274Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_274::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl274Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_274 to value 0"]
impl crate::Resettable for DdrDenaliCtl274Spec {
    const RESET_VALUE: u32 = 0;
}
