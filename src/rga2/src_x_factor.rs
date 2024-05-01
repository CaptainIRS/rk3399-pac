#[doc = "Register `SRC_X_FACTOR` reader"]
pub type R = crate::R<SrcXFactorSpec>;
#[doc = "Register `SRC_X_FACTOR` writer"]
pub type W = crate::W<SrcXFactorSpec>;
#[doc = "Field `SW_SRC_HSD_FACTOR` reader - Source image horizontal down-scaling factor\n\n=(SRC_ACT_WIDTH/DST_ACT_WIDTH) * 65536"]
pub type SwSrcHsdFactorR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC_HSD_FACTOR` writer - Source image horizontal down-scaling factor\n\n=(SRC_ACT_WIDTH/DST_ACT_WIDTH) * 65536"]
pub type SwSrcHsdFactorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_SRC_HSP_FACTOR` reader - Source image horizontal up-scaling factor\n\n=(DST_ACT_WIDTH/SRC_ACT_WIDTH) * 65536"]
pub type SwSrcHspFactorR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC_HSP_FACTOR` writer - Source image horizontal up-scaling factor\n\n=(DST_ACT_WIDTH/SRC_ACT_WIDTH) * 65536"]
pub type SwSrcHspFactorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source image horizontal down-scaling factor\n\n=(SRC_ACT_WIDTH/DST_ACT_WIDTH) * 65536"]
    #[inline(always)]
    pub fn sw_src_hsd_factor(&self) -> SwSrcHsdFactorR {
        SwSrcHsdFactorR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Source image horizontal up-scaling factor\n\n=(DST_ACT_WIDTH/SRC_ACT_WIDTH) * 65536"]
    #[inline(always)]
    pub fn sw_src_hsp_factor(&self) -> SwSrcHspFactorR {
        SwSrcHspFactorR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source image horizontal down-scaling factor\n\n=(SRC_ACT_WIDTH/DST_ACT_WIDTH) * 65536"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_hsd_factor(&mut self) -> SwSrcHsdFactorW<SrcXFactorSpec> {
        SwSrcHsdFactorW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Source image horizontal up-scaling factor\n\n=(DST_ACT_WIDTH/SRC_ACT_WIDTH) * 65536"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_hsp_factor(&mut self) -> SwSrcHspFactorW<SrcXFactorSpec> {
        SwSrcHspFactorW::new(self, 16)
    }
}
#[doc = "RGA source image horizontal scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_x_factor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_x_factor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcXFactorSpec;
impl crate::RegisterSpec for SrcXFactorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_x_factor::R`](R) reader structure"]
impl crate::Readable for SrcXFactorSpec {}
#[doc = "`write(|w| ..)` method takes [`src_x_factor::W`](W) writer structure"]
impl crate::Writable for SrcXFactorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_X_FACTOR to value 0"]
impl crate::Resettable for SrcXFactorSpec {
    const RESET_VALUE: u32 = 0;
}
