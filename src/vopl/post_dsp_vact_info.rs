#[doc = "Register `POST_DSP_VACT_INFO` reader"]
pub type R = crate::R<PostDspVactInfoSpec>;
#[doc = "Register `POST_DSP_VACT_INFO` writer"]
pub type W = crate::W<PostDspVactInfoSpec>;
#[doc = "Field `DSP_VACT_END_POST` reader - Panel display scanning horizontal active end point"]
pub type DspVactEndPostR = crate::FieldReader<u16>;
#[doc = "Field `DSP_VACT_END_POST` writer - Panel display scanning horizontal active end point"]
pub type DspVactEndPostW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_VACT_ST_POST` reader - Panel display scanning horizontal active start point"]
pub type DspVactStPostR = crate::FieldReader<u16>;
#[doc = "Field `DSP_VACT_ST_POST` writer - Panel display scanning horizontal active start point"]
pub type DspVactStPostW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning horizontal active end point"]
    #[inline(always)]
    pub fn dsp_vact_end_post(&self) -> DspVactEndPostR {
        DspVactEndPostR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal active start point"]
    #[inline(always)]
    pub fn dsp_vact_st_post(&self) -> DspVactStPostR {
        DspVactStPostR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning horizontal active end point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vact_end_post(&mut self) -> DspVactEndPostW<PostDspVactInfoSpec> {
        DspVactEndPostW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal active start point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vact_st_post(&mut self) -> DspVactStPostW<PostDspVactInfoSpec> {
        DspVactStPostW::new(self, 16)
    }
}
#[doc = "Panel active horizontal scanning start point and end point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_dsp_vact_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_dsp_vact_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostDspVactInfoSpec;
impl crate::RegisterSpec for PostDspVactInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`post_dsp_vact_info::R`](R) reader structure"]
impl crate::Readable for PostDspVactInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`post_dsp_vact_info::W`](W) writer structure"]
impl crate::Writable for PostDspVactInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POST_DSP_VACT_INFO to value 0x000a_00fa"]
impl crate::Resettable for PostDspVactInfoSpec {
    const RESET_VALUE: u32 = 0x000a_00fa;
}
