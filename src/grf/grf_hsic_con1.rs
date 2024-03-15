#[doc = "Register `GRF_HSIC_CON1` reader"]
pub type R = crate::R<GrfHsicCon1Spec>;
#[doc = "Register `GRF_HSIC_CON1` writer"]
pub type W = crate::W<GrfHsicCon1Spec>;
#[doc = "Field `HSIC_FLADJ` reader - fladj Must set this register to 0x20."]
pub type HsicFladjR = crate::FieldReader;
#[doc = "Field `HSIC_FLADJ` writer - fladj Must set this register to 0x20."]
pub type HsicFladjW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HSIC_FLADJ_VAL_COMMON` reader - fladj_val_common Must set this register to 0x20"]
pub type HsicFladjValCommonR = crate::FieldReader;
#[doc = "Field `HSIC_FLADJ_VAL_COMMON` writer - fladj_val_common Must set this register to 0x20"]
pub type HsicFladjValCommonW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::BitReader;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - fladj Must set this register to 0x20."]
    #[inline(always)]
    pub fn hsic_fladj(&self) -> HsicFladjR {
        HsicFladjR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - fladj_val_common Must set this register to 0x20"]
    #[inline(always)]
    pub fn hsic_fladj_val_common(&self) -> HsicFladjValCommonR {
        HsicFladjValCommonR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - fladj Must set this register to 0x20."]
    #[inline(always)]
    #[must_use]
    pub fn hsic_fladj(&mut self) -> HsicFladjW<GrfHsicCon1Spec> {
        HsicFladjW::new(self, 0)
    }
    #[doc = "Bits 6:11 - fladj_val_common Must set this register to 0x20"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_fladj_val_common(&mut self) -> HsicFladjValCommonW<GrfHsicCon1Spec> {
        HsicFladjValCommonW::new(self, 6)
    }
    #[doc = "Bit 12 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfHsicCon1Spec> {
        WriteEnableW::new(self, 12)
    }
}
#[doc = "HSIC controller GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hsic_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hsic_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfHsicCon1Spec;
impl crate::RegisterSpec for GrfHsicCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_hsic_con1::R`](R) reader structure"]
impl crate::Readable for GrfHsicCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_hsic_con1::W`](W) writer structure"]
impl crate::Writable for GrfHsicCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_HSIC_CON1 to value 0x0820"]
impl crate::Resettable for GrfHsicCon1Spec {
    const RESET_VALUE: u32 = 0x0820;
}
