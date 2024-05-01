#[doc = "Register `SRC_Y_FACTOR` reader"]
pub type R = crate::R<SrcYFactorSpec>;
#[doc = "Register `SRC_Y_FACTOR` writer"]
pub type W = crate::W<SrcYFactorSpec>;
#[doc = "Field `SW_SRC_VSD_FACTOR` reader - Source image vertical down-scaling factor\n\n(SRC_ACT_HEIGHT/DST_ACT_HEIGHT) * 65536"]
pub type SwSrcVsdFactorR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC_VSD_FACTOR` writer - Source image vertical down-scaling factor\n\n(SRC_ACT_HEIGHT/DST_ACT_HEIGHT) * 65536"]
pub type SwSrcVsdFactorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_SRC_VSP_FACTOR` reader - Source image vertical up-scaling factor\n\n(DST_ACT_HEIGHT/SRC_ACT_HEIGHT) * 65536"]
pub type SwSrcVspFactorR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC_VSP_FACTOR` writer - Source image vertical up-scaling factor\n\n(DST_ACT_HEIGHT/SRC_ACT_HEIGHT) * 65536"]
pub type SwSrcVspFactorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source image vertical down-scaling factor\n\n(SRC_ACT_HEIGHT/DST_ACT_HEIGHT) * 65536"]
    #[inline(always)]
    pub fn sw_src_vsd_factor(&self) -> SwSrcVsdFactorR {
        SwSrcVsdFactorR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Source image vertical up-scaling factor\n\n(DST_ACT_HEIGHT/SRC_ACT_HEIGHT) * 65536"]
    #[inline(always)]
    pub fn sw_src_vsp_factor(&self) -> SwSrcVspFactorR {
        SwSrcVspFactorR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source image vertical down-scaling factor\n\n(SRC_ACT_HEIGHT/DST_ACT_HEIGHT) * 65536"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_vsd_factor(&mut self) -> SwSrcVsdFactorW<SrcYFactorSpec> {
        SwSrcVsdFactorW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Source image vertical up-scaling factor\n\n(DST_ACT_HEIGHT/SRC_ACT_HEIGHT) * 65536"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_vsp_factor(&mut self) -> SwSrcVspFactorW<SrcYFactorSpec> {
        SwSrcVspFactorW::new(self, 16)
    }
}
#[doc = "RGA source image vertical scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_y_factor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_y_factor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcYFactorSpec;
impl crate::RegisterSpec for SrcYFactorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_y_factor::R`](R) reader structure"]
impl crate::Readable for SrcYFactorSpec {}
#[doc = "`write(|w| ..)` method takes [`src_y_factor::W`](W) writer structure"]
impl crate::Writable for SrcYFactorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_Y_FACTOR to value 0"]
impl crate::Resettable for SrcYFactorSpec {
    const RESET_VALUE: u32 = 0;
}
