#[doc = "Register `DENALI_CTL_219` reader"]
pub type R = crate::R<DenaliCtl219Spec>;
#[doc = "Register `DENALI_CTL_219` writer"]
pub type W = crate::W<DenaliCtl219Spec>;
#[doc = "Field `R2W_DIFFCS_DLY_F2` reader - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
pub type R2wDiffcsDlyF2R = crate::FieldReader;
#[doc = "Field `R2W_DIFFCS_DLY_F2` writer - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
pub type R2wDiffcsDlyF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2R_DIFFCS_DLY_F2` reader - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
pub type W2rDiffcsDlyF2R = crate::FieldReader;
#[doc = "Field `W2R_DIFFCS_DLY_F2` writer - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
pub type W2rDiffcsDlyF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2W_DIFFCS_DLY_F2` reader - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
pub type W2wDiffcsDlyF2R = crate::FieldReader;
#[doc = "Field `W2W_DIFFCS_DLY_F2` writer - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
pub type W2wDiffcsDlyF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R2R_SAMECS_DLY` reader - Additional delay to insert between two reads to the same chip select. Any value including 0x0 supported."]
pub type R2rSamecsDlyR = crate::FieldReader;
#[doc = "Field `R2R_SAMECS_DLY` writer - Additional delay to insert between two reads to the same chip select. Any value including 0x0 supported."]
pub type R2rSamecsDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    pub fn r2w_diffcs_dly_f2(&self) -> R2wDiffcsDlyF2R {
        R2wDiffcsDlyF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    pub fn w2r_diffcs_dly_f2(&self) -> W2rDiffcsDlyF2R {
        W2rDiffcsDlyF2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    pub fn w2w_diffcs_dly_f2(&self) -> W2wDiffcsDlyF2R {
        W2wDiffcsDlyF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between two reads to the same chip select. Any value including 0x0 supported."]
    #[inline(always)]
    pub fn r2r_samecs_dly(&self) -> R2rSamecsDlyR {
        R2rSamecsDlyR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn r2w_diffcs_dly_f2(&mut self) -> R2wDiffcsDlyF2W<DenaliCtl219Spec> {
        R2wDiffcsDlyF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    #[must_use]
    pub fn w2r_diffcs_dly_f2(&mut self) -> W2rDiffcsDlyF2W<DenaliCtl219Spec> {
        W2rDiffcsDlyF2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn w2w_diffcs_dly_f2(&mut self) -> W2wDiffcsDlyF2W<DenaliCtl219Spec> {
        W2wDiffcsDlyF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between two reads to the same chip select. Any value including 0x0 supported."]
    #[inline(always)]
    #[must_use]
    pub fn r2r_samecs_dly(&mut self) -> R2rSamecsDlyW<DenaliCtl219Spec> {
        R2rSamecsDlyW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_219::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_219::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl219Spec;
impl crate::RegisterSpec for DenaliCtl219Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_219::R`](R) reader structure"]
impl crate::Readable for DenaliCtl219Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_219::W`](W) writer structure"]
impl crate::Writable for DenaliCtl219Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_219 to value 0x0001_0101"]
impl crate::Resettable for DenaliCtl219Spec {
    const RESET_VALUE: u32 = 0x0001_0101;
}
