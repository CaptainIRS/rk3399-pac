#[doc = "Register `EXP_H_OFFSET` reader"]
pub type R = crate::R<ExpHOffsetSpec>;
#[doc = "Register `EXP_H_OFFSET` writer"]
pub type W = crate::W<ExpHOffsetSpec>;
#[doc = "Field `isp_exp_h_offset` reader - Horizontal offset of first block in pixels. 0 &lt;= value &lt;=2424"]
pub type IspExpHOffsetR = crate::FieldReader<u16>;
#[doc = "Field `isp_exp_h_offset` writer - Horizontal offset of first block in pixels. 0 &lt;= value &lt;=2424"]
pub type IspExpHOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal offset of first block in pixels. 0 &lt;= value &lt;=2424"]
    #[inline(always)]
    pub fn isp_exp_h_offset(&self) -> IspExpHOffsetR {
        IspExpHOffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal offset of first block in pixels. 0 &lt;= value &lt;=2424"]
    #[inline(always)]
    #[must_use]
    pub fn isp_exp_h_offset(&mut self) -> IspExpHOffsetW<ExpHOffsetSpec> {
        IspExpHOffsetW::new(self, 0)
    }
}
#[doc = "Horizontal offset for first block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_h_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_h_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpHOffsetSpec;
impl crate::RegisterSpec for ExpHOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_h_offset::R`](R) reader structure"]
impl crate::Readable for ExpHOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`exp_h_offset::W`](W) writer structure"]
impl crate::Writable for ExpHOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXP_H_OFFSET to value 0"]
impl crate::Resettable for ExpHOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
