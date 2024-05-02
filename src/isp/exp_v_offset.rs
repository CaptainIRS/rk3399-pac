#[doc = "Register `EXP_V_OFFSET` reader"]
pub type R = crate::R<ExpVOffsetSpec>;
#[doc = "Register `EXP_V_OFFSET` writer"]
pub type W = crate::W<ExpVOffsetSpec>;
#[doc = "Field `isp_exp_v_offset` reader - Vertical offset of first block\n\nin pixels. 0 &lt;= value &lt;= 1806\n\n"]
pub type IspExpVOffsetR = crate::FieldReader<u16>;
#[doc = "Field `isp_exp_v_offset` writer - Vertical offset of first block\n\nin pixels. 0 &lt;= value &lt;= 1806\n\n"]
pub type IspExpVOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Vertical offset of first block\n\nin pixels. 0 &lt;= value &lt;= 1806\n\n"]
    #[inline(always)]
    pub fn isp_exp_v_offset(&self) -> IspExpVOffsetR {
        IspExpVOffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical offset of first block\n\nin pixels. 0 &lt;= value &lt;= 1806\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn isp_exp_v_offset(&mut self) -> IspExpVOffsetW<ExpVOffsetSpec> {
        IspExpVOffsetW::new(self, 0)
    }
}
#[doc = "Vertical offset for first block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_v_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_v_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpVOffsetSpec;
impl crate::RegisterSpec for ExpVOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_v_offset::R`](R) reader structure"]
impl crate::Readable for ExpVOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`exp_v_offset::W`](W) writer structure"]
impl crate::Writable for ExpVOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXP_V_OFFSET to value 0"]
impl crate::Resettable for ExpVOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
