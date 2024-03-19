#[doc = "Register `FC_AUDSV` reader"]
pub type R = crate::R<FcAudsvSpec>;
#[doc = "Register `FC_AUDSV` writer"]
pub type W = crate::W<FcAudsvSpec>;
#[doc = "Field `V0L` reader - Set validity bit \"V\" for Channel 0, Left"]
pub type V0lR = crate::BitReader;
#[doc = "Field `V0L` writer - Set validity bit \"V\" for Channel 0, Left"]
pub type V0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V1L` reader - Set validity bit \"V\" for Channel 1, Left"]
pub type V1lR = crate::BitReader;
#[doc = "Field `V1L` writer - Set validity bit \"V\" for Channel 1, Left"]
pub type V1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V2L` reader - Set validity bit \"V\" for Channel 2, Left"]
pub type V2lR = crate::BitReader;
#[doc = "Field `V2L` writer - Set validity bit \"V\" for Channel 2, Left"]
pub type V2lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V3L` reader - Set validity bit \"V\" for Channel 3, Left"]
pub type V3lR = crate::BitReader;
#[doc = "Field `V3L` writer - Set validity bit \"V\" for Channel 3, Left"]
pub type V3lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V0R` reader - Set validity bit \"V\" for Channel 0, Right"]
pub type V0rR = crate::BitReader;
#[doc = "Field `V0R` writer - Set validity bit \"V\" for Channel 0, Right"]
pub type V0rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V1R` reader - Set validity bit \"V\" for Channel 1, Right"]
pub type V1rR = crate::BitReader;
#[doc = "Field `V1R` writer - Set validity bit \"V\" for Channel 1, Right"]
pub type V1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V2R` reader - Set validity bit \"V\" for Channel 2, Right"]
pub type V2rR = crate::BitReader;
#[doc = "Field `V2R` writer - Set validity bit \"V\" for Channel 2, Right"]
pub type V2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V3R` reader - Set validity bit \"V\" for Channel 3, Right"]
pub type V3rR = crate::BitReader;
#[doc = "Field `V3R` writer - Set validity bit \"V\" for Channel 3, Right"]
pub type V3rW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set validity bit \"V\" for Channel 0, Left"]
    #[inline(always)]
    pub fn v0l(&self) -> V0lR {
        V0lR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set validity bit \"V\" for Channel 1, Left"]
    #[inline(always)]
    pub fn v1l(&self) -> V1lR {
        V1lR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set validity bit \"V\" for Channel 2, Left"]
    #[inline(always)]
    pub fn v2l(&self) -> V2lR {
        V2lR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set validity bit \"V\" for Channel 3, Left"]
    #[inline(always)]
    pub fn v3l(&self) -> V3lR {
        V3lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set validity bit \"V\" for Channel 0, Right"]
    #[inline(always)]
    pub fn v0r(&self) -> V0rR {
        V0rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set validity bit \"V\" for Channel 1, Right"]
    #[inline(always)]
    pub fn v1r(&self) -> V1rR {
        V1rR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set validity bit \"V\" for Channel 2, Right"]
    #[inline(always)]
    pub fn v2r(&self) -> V2rR {
        V2rR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set validity bit \"V\" for Channel 3, Right"]
    #[inline(always)]
    pub fn v3r(&self) -> V3rR {
        V3rR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set validity bit \"V\" for Channel 0, Left"]
    #[inline(always)]
    #[must_use]
    pub fn v0l(&mut self) -> V0lW<FcAudsvSpec> {
        V0lW::new(self, 0)
    }
    #[doc = "Bit 1 - Set validity bit \"V\" for Channel 1, Left"]
    #[inline(always)]
    #[must_use]
    pub fn v1l(&mut self) -> V1lW<FcAudsvSpec> {
        V1lW::new(self, 1)
    }
    #[doc = "Bit 2 - Set validity bit \"V\" for Channel 2, Left"]
    #[inline(always)]
    #[must_use]
    pub fn v2l(&mut self) -> V2lW<FcAudsvSpec> {
        V2lW::new(self, 2)
    }
    #[doc = "Bit 3 - Set validity bit \"V\" for Channel 3, Left"]
    #[inline(always)]
    #[must_use]
    pub fn v3l(&mut self) -> V3lW<FcAudsvSpec> {
        V3lW::new(self, 3)
    }
    #[doc = "Bit 4 - Set validity bit \"V\" for Channel 0, Right"]
    #[inline(always)]
    #[must_use]
    pub fn v0r(&mut self) -> V0rW<FcAudsvSpec> {
        V0rW::new(self, 4)
    }
    #[doc = "Bit 5 - Set validity bit \"V\" for Channel 1, Right"]
    #[inline(always)]
    #[must_use]
    pub fn v1r(&mut self) -> V1rW<FcAudsvSpec> {
        V1rW::new(self, 5)
    }
    #[doc = "Bit 6 - Set validity bit \"V\" for Channel 2, Right"]
    #[inline(always)]
    #[must_use]
    pub fn v2r(&mut self) -> V2rW<FcAudsvSpec> {
        V2rW::new(self, 6)
    }
    #[doc = "Bit 7 - Set validity bit \"V\" for Channel 3, Right"]
    #[inline(always)]
    #[must_use]
    pub fn v3r(&mut self) -> V3rW<FcAudsvSpec> {
        V3rW::new(self, 7)
    }
}
#[doc = "Frame Composer Audio Sample Validity Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audsv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudsvSpec;
impl crate::RegisterSpec for FcAudsvSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audsv::R`](R) reader structure"]
impl crate::Readable for FcAudsvSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_audsv::W`](W) writer structure"]
impl crate::Writable for FcAudsvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSV to value 0"]
impl crate::Resettable for FcAudsvSpec {
    const RESET_VALUE: u8 = 0;
}
