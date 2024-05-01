#[doc = "Register `CABC_CTRL1` reader"]
pub type R = crate::R<CabcCtrl1Spec>;
#[doc = "Register `CABC_CTRL1` writer"]
pub type W = crate::W<CabcCtrl1Spec>;
#[doc = "Field `CABC_LUT_EN` reader - cabc pwm lut enable"]
pub type CabcLutEnR = crate::BitReader;
#[doc = "Field `CABC_LUT_EN` writer - cabc pwm lut enable"]
pub type CabcLutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CABC_TOTAL_NUM` reader - cabc totala numbers = h_vd * v_vd"]
pub type CabcTotalNumR = crate::FieldReader<u32>;
#[doc = "Field `CABC_TOTAL_NUM` writer - cabc totala numbers = h_vd * v_vd"]
pub type CabcTotalNumW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - cabc pwm lut enable"]
    #[inline(always)]
    pub fn cabc_lut_en(&self) -> CabcLutEnR {
        CabcLutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:26 - cabc totala numbers = h_vd * v_vd"]
    #[inline(always)]
    pub fn cabc_total_num(&self) -> CabcTotalNumR {
        CabcTotalNumR::new((self.bits >> 4) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - cabc pwm lut enable"]
    #[inline(always)]
    #[must_use]
    pub fn cabc_lut_en(&mut self) -> CabcLutEnW<CabcCtrl1Spec> {
        CabcLutEnW::new(self, 0)
    }
    #[doc = "Bits 4:26 - cabc totala numbers = h_vd * v_vd"]
    #[inline(always)]
    #[must_use]
    pub fn cabc_total_num(&mut self) -> CabcTotalNumW<CabcCtrl1Spec> {
        CabcTotalNumW::new(self, 4)
    }
}
#[doc = "Content Adaptive Backlight Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcCtrl1Spec;
impl crate::RegisterSpec for CabcCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_ctrl1::R`](R) reader structure"]
impl crate::Readable for CabcCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_ctrl1::W`](W) writer structure"]
impl crate::Writable for CabcCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_CTRL1 to value 0x00fa_0000"]
impl crate::Resettable for CabcCtrl1Spec {
    const RESET_VALUE: u32 = 0x00fa_0000;
}
