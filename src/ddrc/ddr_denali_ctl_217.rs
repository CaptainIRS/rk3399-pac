#[doc = "Register `DDR_DENALI_CTL_217` reader"]
pub type R = crate::R<DdrDenaliCtl217Spec>;
#[doc = "Register `DDR_DENALI_CTL_217` writer"]
pub type W = crate::W<DdrDenaliCtl217Spec>;
#[doc = "Field `R2W_DIFFCS_DLY_F0` reader - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
pub type R2wDiffcsDlyF0R = crate::FieldReader;
#[doc = "Field `R2W_DIFFCS_DLY_F0` writer - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
pub type R2wDiffcsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2R_DIFFCS_DLY_F0` reader - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
pub type W2rDiffcsDlyF0R = crate::FieldReader;
#[doc = "Field `W2R_DIFFCS_DLY_F0` writer - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
pub type W2rDiffcsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2W_DIFFCS_DLY_F0` reader - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
pub type W2wDiffcsDlyF0R = crate::FieldReader;
#[doc = "Field `W2W_DIFFCS_DLY_F0` writer - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
pub type W2wDiffcsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R2R_DIFFCS_DLY_F1` reader - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
pub type R2rDiffcsDlyF1R = crate::FieldReader;
#[doc = "Field `R2R_DIFFCS_DLY_F1` writer - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
pub type R2rDiffcsDlyF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    pub fn r2w_diffcs_dly_f0(&self) -> R2wDiffcsDlyF0R {
        R2wDiffcsDlyF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    pub fn w2r_diffcs_dly_f0(&self) -> W2rDiffcsDlyF0R {
        W2rDiffcsDlyF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    pub fn w2w_diffcs_dly_f0(&self) -> W2wDiffcsDlyF0R {
        W2wDiffcsDlyF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    pub fn r2r_diffcs_dly_f1(&self) -> R2rDiffcsDlyF1R {
        R2rDiffcsDlyF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn r2w_diffcs_dly_f0(&mut self) -> R2wDiffcsDlyF0W<DdrDenaliCtl217Spec> {
        R2wDiffcsDlyF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    #[must_use]
    pub fn w2r_diffcs_dly_f0(&mut self) -> W2rDiffcsDlyF0W<DdrDenaliCtl217Spec> {
        W2rDiffcsDlyF0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Additional delay to insert between writes to different chip selects. Program to a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn w2w_diffcs_dly_f0(&mut self) -> W2wDiffcsDlyF0W<DdrDenaliCtl217Spec> {
        W2wDiffcsDlyF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    #[must_use]
    pub fn r2r_diffcs_dly_f1(&mut self) -> R2rDiffcsDlyF1W<DdrDenaliCtl217Spec> {
        R2rDiffcsDlyF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_217::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_217::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl217Spec;
impl crate::RegisterSpec for DdrDenaliCtl217Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_217::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl217Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_217::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl217Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_217 to value 0x0101_0101"]
impl crate::Resettable for DdrDenaliCtl217Spec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
