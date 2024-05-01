#[doc = "Register `POST_DSP_HACT_INFO` reader"]
pub type R = crate::R<PostDspHactInfoSpec>;
#[doc = "Register `POST_DSP_HACT_INFO` writer"]
pub type W = crate::W<PostDspHactInfoSpec>;
#[doc = "Field `DSP_HACT_END_POST` reader - Panel display scanning horizontal active end point"]
pub type DspHactEndPostR = crate::FieldReader<u16>;
#[doc = "Field `DSP_HACT_END_POST` writer - Panel display scanning horizontal active end point"]
pub type DspHactEndPostW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DSP_HACT_ST_POST` reader - Panel display scanning horizontal active start point"]
pub type DspHactStPostR = crate::FieldReader<u16>;
#[doc = "Field `DSP_HACT_ST_POST` writer - Panel display scanning horizontal active start point"]
pub type DspHactStPostW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Panel display scanning horizontal active end point"]
    #[inline(always)]
    pub fn dsp_hact_end_post(&self) -> DspHactEndPostR {
        DspHactEndPostR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal active start point"]
    #[inline(always)]
    pub fn dsp_hact_st_post(&self) -> DspHactStPostR {
        DspHactStPostR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Panel display scanning horizontal active end point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_hact_end_post(&mut self) -> DspHactEndPostW<PostDspHactInfoSpec> {
        DspHactEndPostW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Panel display scanning horizontal active start point"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_hact_st_post(&mut self) -> DspHactStPostW<PostDspHactInfoSpec> {
        DspHactStPostW::new(self, 16)
    }
}
#[doc = "Post scaler down horizontal start and end\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_dsp_hact_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_dsp_hact_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostDspHactInfoSpec;
impl crate::RegisterSpec for PostDspHactInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`post_dsp_hact_info::R`](R) reader structure"]
impl crate::Readable for PostDspHactInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`post_dsp_hact_info::W`](W) writer structure"]
impl crate::Writable for PostDspHactInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POST_DSP_HACT_INFO to value 0x000a_014a"]
impl crate::Resettable for PostDspHactInfoSpec {
    const RESET_VALUE: u32 = 0x000a_014a;
}
