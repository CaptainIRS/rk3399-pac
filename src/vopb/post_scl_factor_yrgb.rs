#[doc = "Register `POST_SCL_FACTOR_YRGB` reader"]
pub type R = crate::R<PostSclFactorYrgbSpec>;
#[doc = "Register `POST_SCL_FACTOR_YRGB` writer"]
pub type W = crate::W<PostSclFactorYrgbSpec>;
#[doc = "Field `POST_HS_FACTOR_YRGB` reader - Post YRGB horizontal scaling factor:\n\nfactor=((src_width\\[15:0\\])\n\n/(dst_width\\[15:0\\]))*2^12"]
pub type PostHsFactorYrgbR = crate::FieldReader<u16>;
#[doc = "Field `POST_HS_FACTOR_YRGB` writer - Post YRGB horizontal scaling factor:\n\nfactor=((src_width\\[15:0\\])\n\n/(dst_width\\[15:0\\]))*2^12"]
pub type PostHsFactorYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `POST_VS_FACTOR_YRGB` reader - post YRGB vertical scaling factor:\n\nfactor=((src_height\\[31:16\\])\n\n/(dst_height\\[31:16\\]))*2^12"]
pub type PostVsFactorYrgbR = crate::FieldReader<u16>;
#[doc = "Field `POST_VS_FACTOR_YRGB` writer - post YRGB vertical scaling factor:\n\nfactor=((src_height\\[31:16\\])\n\n/(dst_height\\[31:16\\]))*2^12"]
pub type PostVsFactorYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Post YRGB horizontal scaling factor:\n\nfactor=((src_width\\[15:0\\])\n\n/(dst_width\\[15:0\\]))*2^12"]
    #[inline(always)]
    pub fn post_hs_factor_yrgb(&self) -> PostHsFactorYrgbR {
        PostHsFactorYrgbR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - post YRGB vertical scaling factor:\n\nfactor=((src_height\\[31:16\\])\n\n/(dst_height\\[31:16\\]))*2^12"]
    #[inline(always)]
    pub fn post_vs_factor_yrgb(&self) -> PostVsFactorYrgbR {
        PostVsFactorYrgbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Post YRGB horizontal scaling factor:\n\nfactor=((src_width\\[15:0\\])\n\n/(dst_width\\[15:0\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn post_hs_factor_yrgb(&mut self) -> PostHsFactorYrgbW<PostSclFactorYrgbSpec> {
        PostHsFactorYrgbW::new(self, 0)
    }
    #[doc = "Bits 16:31 - post YRGB vertical scaling factor:\n\nfactor=((src_height\\[31:16\\])\n\n/(dst_height\\[31:16\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn post_vs_factor_yrgb(&mut self) -> PostVsFactorYrgbW<PostSclFactorYrgbSpec> {
        PostVsFactorYrgbW::new(self, 16)
    }
}
#[doc = "Post yrgb scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_scl_factor_yrgb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_scl_factor_yrgb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostSclFactorYrgbSpec;
impl crate::RegisterSpec for PostSclFactorYrgbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`post_scl_factor_yrgb::R`](R) reader structure"]
impl crate::Readable for PostSclFactorYrgbSpec {}
#[doc = "`write(|w| ..)` method takes [`post_scl_factor_yrgb::W`](W) writer structure"]
impl crate::Writable for PostSclFactorYrgbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POST_SCL_FACTOR_YRGB to value 0x1000_1000"]
impl crate::Resettable for PostSclFactorYrgbSpec {
    const RESET_VALUE: u32 = 0x1000_1000;
}
