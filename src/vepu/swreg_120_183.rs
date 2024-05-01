#[doc = "Register `SWREG_120_183` writer"]
pub type W = crate::W<Swreg120_183Spec>;
#[doc = "Field `DMV_PLY_TABLE` writer - DMV 4p/1p penalty table values\n\naddr range : 0x01e0~0x02dc\n\nswreg120: DMV 4p/1p penalty table values\n\nswreg121: DMV 4p/1p penalty table values\n\nswreg122: DMV 4p/1p penalty table values\n\nswreg123: DMV 4p/1p penalty table values\n\n.........\n\nswreg183: DMV 4p/1p penalty table values"]
pub type DmvPlyTableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - DMV 4p/1p penalty table values\n\naddr range : 0x01e0~0x02dc\n\nswreg120: DMV 4p/1p penalty table values\n\nswreg121: DMV 4p/1p penalty table values\n\nswreg122: DMV 4p/1p penalty table values\n\nswreg123: DMV 4p/1p penalty table values\n\n.........\n\nswreg183: DMV 4p/1p penalty table values"]
    #[inline(always)]
    #[must_use]
    pub fn dmv_ply_table(&mut self) -> DmvPlyTableW<Swreg120_183Spec> {
        DmvPlyTableW::new(self, 0)
    }
}
#[doc = "DMV_4p_1p_penalty\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_120_183::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg120_183Spec;
impl crate::RegisterSpec for Swreg120_183Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreg_120_183::W`](W) writer structure"]
impl crate::Writable for Swreg120_183Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_120_183 to value 0"]
impl crate::Resettable for Swreg120_183Spec {
    const RESET_VALUE: u32 = 0;
}
