#[doc = "Register `DDR_DENALI_CTL_220` reader"]
pub type R = crate::R<DdrDenaliCtl220Spec>;
#[doc = "Register `DDR_DENALI_CTL_220` writer"]
pub type W = crate::W<DdrDenaliCtl220Spec>;
#[doc = "Field `R2W_SAMECS_DLY_F0` reader - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
pub type R2wSamecsDlyF0R = crate::FieldReader;
#[doc = "Field `R2W_SAMECS_DLY_F0` writer - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
pub type R2wSamecsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R2W_SAMECS_DLY_F1` reader - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
pub type R2wSamecsDlyF1R = crate::FieldReader;
#[doc = "Field `R2W_SAMECS_DLY_F1` writer - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
pub type R2wSamecsDlyF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R2W_SAMECS_DLY_F2` reader - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
pub type R2wSamecsDlyF2R = crate::FieldReader;
#[doc = "Field `R2W_SAMECS_DLY_F2` writer - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
pub type R2wSamecsDlyF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2R_SAMECS_DLY` reader - Additional delay to insert between writes and reads to the same chip select."]
pub type W2rSamecsDlyR = crate::FieldReader;
#[doc = "Field `W2R_SAMECS_DLY` writer - Additional delay to insert between writes and reads to the same chip select."]
pub type W2rSamecsDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
    #[inline(always)]
    pub fn r2w_samecs_dly_f0(&self) -> R2wSamecsDlyF0R {
        R2wSamecsDlyF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
    #[inline(always)]
    pub fn r2w_samecs_dly_f1(&self) -> R2wSamecsDlyF1R {
        R2wSamecsDlyF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
    #[inline(always)]
    pub fn r2w_samecs_dly_f2(&self) -> R2wSamecsDlyF2R {
        R2wSamecsDlyF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between writes and reads to the same chip select."]
    #[inline(always)]
    pub fn w2r_samecs_dly(&self) -> W2rSamecsDlyR {
        W2rSamecsDlyR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn r2w_samecs_dly_f0(&mut self) -> R2wSamecsDlyF0W<DdrDenaliCtl220Spec> {
        R2wSamecsDlyF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn r2w_samecs_dly_f1(&mut self) -> R2wSamecsDlyF1W<DdrDenaliCtl220Spec> {
        R2wSamecsDlyF1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Additional delay to insert between reads and writes to the same chip select. Program to a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn r2w_samecs_dly_f2(&mut self) -> R2wSamecsDlyF2W<DdrDenaliCtl220Spec> {
        R2wSamecsDlyF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between writes and reads to the same chip select."]
    #[inline(always)]
    #[must_use]
    pub fn w2r_samecs_dly(&mut self) -> W2rSamecsDlyW<DdrDenaliCtl220Spec> {
        W2rSamecsDlyW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_220::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_220::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl220Spec;
impl crate::RegisterSpec for DdrDenaliCtl220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_220::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl220Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_220::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl220Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_220 to value 0x0002_0202"]
impl crate::Resettable for DdrDenaliCtl220Spec {
    const RESET_VALUE: u32 = 0x0002_0202;
}
