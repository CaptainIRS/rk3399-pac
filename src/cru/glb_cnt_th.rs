#[doc = "Register `GLB_CNT_TH` reader"]
pub type R = crate::R<GlbCntThSpec>;
#[doc = "Register `GLB_CNT_TH` writer"]
pub type W = crate::W<GlbCntThSpec>;
#[doc = "Field `GLB_RST_CNT_TH` reader - global reset wait counter threshold\n\nwait cycles n(at xin_24m)"]
pub type GlbRstCntThR = crate::FieldReader<u16>;
#[doc = "Field `GLB_RST_CNT_TH` writer - global reset wait counter threshold\n\nwait cycles n(at xin_24m)"]
pub type GlbRstCntThW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - global reset wait counter threshold\n\nwait cycles n(at xin_24m)"]
    #[inline(always)]
    pub fn glb_rst_cnt_th(&self) -> GlbRstCntThR {
        GlbRstCntThR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - global reset wait counter threshold\n\nwait cycles n(at xin_24m)"]
    #[inline(always)]
    #[must_use]
    pub fn glb_rst_cnt_th(&mut self) -> GlbRstCntThW<GlbCntThSpec> {
        GlbRstCntThW::new(self, 0)
    }
}
#[doc = "Global soft reset counter threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_cnt_th::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_cnt_th::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbCntThSpec;
impl crate::RegisterSpec for GlbCntThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_cnt_th::R`](R) reader structure"]
impl crate::Readable for GlbCntThSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_cnt_th::W`](W) writer structure"]
impl crate::Writable for GlbCntThSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_CNT_TH to value 0"]
impl crate::Resettable for GlbCntThSpec {
    const RESET_VALUE: u32 = 0;
}
