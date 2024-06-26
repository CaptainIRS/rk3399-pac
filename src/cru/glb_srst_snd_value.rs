#[doc = "Register `GLB_SRST_SND_VALUE` reader"]
pub type R = crate::R<GlbSrstSndValueSpec>;
#[doc = "Register `GLB_SRST_SND_VALUE` writer"]
pub type W = crate::W<GlbSrstSndValueSpec>;
#[doc = "Field `GLB_SRST_SND_VALUE` reader - The second global software reset config value\n\nIf config 0xeca8, it will generate second global software reset"]
pub type GlbSrstSndValueR = crate::FieldReader<u16>;
#[doc = "Field `GLB_SRST_SND_VALUE` writer - The second global software reset config value\n\nIf config 0xeca8, it will generate second global software reset"]
pub type GlbSrstSndValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The second global software reset config value\n\nIf config 0xeca8, it will generate second global software reset"]
    #[inline(always)]
    pub fn glb_srst_snd_value(&self) -> GlbSrstSndValueR {
        GlbSrstSndValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The second global software reset config value\n\nIf config 0xeca8, it will generate second global software reset"]
    #[inline(always)]
    #[must_use]
    pub fn glb_srst_snd_value(&mut self) -> GlbSrstSndValueW<GlbSrstSndValueSpec> {
        GlbSrstSndValueW::new(self, 0)
    }
}
#[doc = "The second global software reset config value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_srst_snd_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_srst_snd_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbSrstSndValueSpec;
impl crate::RegisterSpec for GlbSrstSndValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_srst_snd_value::R`](R) reader structure"]
impl crate::Readable for GlbSrstSndValueSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_srst_snd_value::W`](W) writer structure"]
impl crate::Writable for GlbSrstSndValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_SRST_SND_VALUE to value 0"]
impl crate::Resettable for GlbSrstSndValueSpec {
    const RESET_VALUE: u32 = 0;
}
