#[doc = "Register `DENALI_CTL_216` reader"]
pub type R = crate::R<DenaliCtl216Spec>;
#[doc = "Register `DENALI_CTL_216` writer"]
pub type W = crate::W<DenaliCtl216Spec>;
#[doc = "Field `RW2MRW_DLY_F0` reader - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
pub type Rw2mrwDlyF0R = crate::FieldReader;
#[doc = "Field `RW2MRW_DLY_F0` writer - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
pub type Rw2mrwDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RW2MRW_DLY_F1` reader - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
pub type Rw2mrwDlyF1R = crate::FieldReader;
#[doc = "Field `RW2MRW_DLY_F1` writer - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
pub type Rw2mrwDlyF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RW2MRW_DLY_F2` reader - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
pub type Rw2mrwDlyF2R = crate::FieldReader;
#[doc = "Field `RW2MRW_DLY_F2` writer - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
pub type Rw2mrwDlyF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `R2R_DIFFCS_DLY_F0` reader - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
pub type R2rDiffcsDlyF0R = crate::FieldReader;
#[doc = "Field `R2R_DIFFCS_DLY_F0` writer - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
pub type R2rDiffcsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
    #[inline(always)]
    pub fn rw2mrw_dly_f0(&self) -> Rw2mrwDlyF0R {
        Rw2mrwDlyF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
    #[inline(always)]
    pub fn rw2mrw_dly_f1(&self) -> Rw2mrwDlyF1R {
        Rw2mrwDlyF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
    #[inline(always)]
    pub fn rw2mrw_dly_f2(&self) -> Rw2mrwDlyF2R {
        Rw2mrwDlyF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    pub fn r2r_diffcs_dly_f0(&self) -> R2rDiffcsDlyF0R {
        R2rDiffcsDlyF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
    #[inline(always)]
    #[must_use]
    pub fn rw2mrw_dly_f0(&mut self) -> Rw2mrwDlyF0W<DenaliCtl216Spec> {
        Rw2mrwDlyF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
    #[inline(always)]
    #[must_use]
    pub fn rw2mrw_dly_f1(&mut self) -> Rw2mrwDlyF1W<DenaliCtl216Spec> {
        Rw2mrwDlyF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system."]
    #[inline(always)]
    #[must_use]
    pub fn rw2mrw_dly_f2(&mut self) -> Rw2mrwDlyF2W<DenaliCtl216Spec> {
        Rw2mrwDlyF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Additional delay to insert between reads to different chip selects. Allowed programming dependent on memory system."]
    #[inline(always)]
    #[must_use]
    pub fn r2r_diffcs_dly_f0(&mut self) -> R2rDiffcsDlyF0W<DenaliCtl216Spec> {
        R2rDiffcsDlyF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_216::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_216::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl216Spec;
impl crate::RegisterSpec for DenaliCtl216Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_216::R`](R) reader structure"]
impl crate::Readable for DenaliCtl216Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_216::W`](W) writer structure"]
impl crate::Writable for DenaliCtl216Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_216 to value 0x0108_0808"]
impl crate::Resettable for DenaliCtl216Spec {
    const RESET_VALUE: u32 = 0x0108_0808;
}
