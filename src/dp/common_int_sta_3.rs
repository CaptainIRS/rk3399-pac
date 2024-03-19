#[doc = "Register `COMMON_INT_STA_3` reader"]
pub type R = crate::R<CommonIntSta3Spec>;
#[doc = "Register `COMMON_INT_STA_3` writer"]
pub type W = crate::W<CommonIntSta3Spec>;
#[doc = "Field `MYDP_HPD_IRQ` reader - 1: MYDP HPD interrupt is detected. \n\nWrite 1 to clear"]
pub type MydpHpdIrqR = crate::BitReader;
#[doc = "Field `MYDP_HPD_IRQ` writer - 1: MYDP HPD interrupt is detected. \n\nWrite 1 to clear"]
pub type MydpHpdIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MYDP_PLUG_OUT` reader - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
pub type MydpPlugOutR = crate::BitReader;
#[doc = "Field `MYDP_PLUG_OUT` writer - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
pub type MydpPlugOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MYDP_PLUG_IN` reader - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
pub type MydpPlugInR = crate::BitReader;
#[doc = "Field `MYDP_PLUG_IN` writer - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
pub type MydpPlugInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPCD_SPECIFIC_IRQ` reader - 1: Sink specific interrupt in DPCD is \n\ndetected. \n\nWrite 1 to clear"]
pub type DpcdSpecificIrqR = crate::BitReader;
#[doc = "Field `DPCD_SPECIFIC_IRQ` writer - 1: Sink specific interrupt in DPCD is \n\ndetected. \n\nWrite 1 to clear"]
pub type DpcdSpecificIrqW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1: MYDP HPD interrupt is detected. \n\nWrite 1 to clear"]
    #[inline(always)]
    pub fn mydp_hpd_irq(&self) -> MydpHpdIrqR {
        MydpHpdIrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
    #[inline(always)]
    pub fn mydp_plug_out(&self) -> MydpPlugOutR {
        MydpPlugOutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
    #[inline(always)]
    pub fn mydp_plug_in(&self) -> MydpPlugInR {
        MydpPlugInR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Sink specific interrupt in DPCD is \n\ndetected. \n\nWrite 1 to clear"]
    #[inline(always)]
    pub fn dpcd_specific_irq(&self) -> DpcdSpecificIrqR {
        DpcdSpecificIrqR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1: MYDP HPD interrupt is detected. \n\nWrite 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn mydp_hpd_irq(&mut self) -> MydpHpdIrqW<CommonIntSta3Spec> {
        MydpHpdIrqW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn mydp_plug_out(&mut self) -> MydpPlugOutW<CommonIntSta3Spec> {
        MydpPlugOutW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: MYDP plug out event is detected. \n\nWrite 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn mydp_plug_in(&mut self) -> MydpPlugInW<CommonIntSta3Spec> {
        MydpPlugInW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Sink specific interrupt in DPCD is \n\ndetected. \n\nWrite 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn dpcd_specific_irq(&mut self) -> DpcdSpecificIrqW<CommonIntSta3Spec> {
        DpcdSpecificIrqW::new(self, 4)
    }
}
#[doc = "Common Interrupt Status Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_sta_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_sta_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonIntSta3Spec;
impl crate::RegisterSpec for CommonIntSta3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`common_int_sta_3::R`](R) reader structure"]
impl crate::Readable for CommonIntSta3Spec {}
#[doc = "`write(|w| ..)` method takes [`common_int_sta_3::W`](W) writer structure"]
impl crate::Writable for CommonIntSta3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets COMMON_INT_STA_3 to value 0"]
impl crate::Resettable for CommonIntSta3Spec {
    const RESET_VALUE: u32 = 0;
}
