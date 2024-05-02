#[doc = "Register `DPCC_RND_OFFS` reader"]
pub type R = crate::R<DpccRndOffsSpec>;
#[doc = "Register `DPCC_RND_OFFS` writer"]
pub type W = crate::W<DpccRndOffsSpec>;
#[doc = "Field `RND_OFFS_1_G` reader - Rank Offset to Neighbor for set 1 green"]
pub type RndOffs1GR = crate::FieldReader;
#[doc = "Field `RND_OFFS_1_G` writer - Rank Offset to Neighbor for set 1 green"]
pub type RndOffs1GW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RND_OFFS_1_RB` reader - Rank Offset to Neighbor for set 1 red/blue"]
pub type RndOffs1RbR = crate::FieldReader;
#[doc = "Field `RND_OFFS_1_RB` writer - Rank Offset to Neighbor for set 1 red/blue"]
pub type RndOffs1RbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RND_OFFS_2_G` reader - Rank Offset to Neighbor for set 2 green"]
pub type RndOffs2GR = crate::FieldReader;
#[doc = "Field `RND_OFFS_2_G` writer - Rank Offset to Neighbor for set 2 green"]
pub type RndOffs2GW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RND_OFFS_2_RB` reader - Rank Offset to Neighbor for set 2 red/blue"]
pub type RndOffs2RbR = crate::FieldReader;
#[doc = "Field `RND_OFFS_2_RB` writer - Rank Offset to Neighbor for set 2 red/blue"]
pub type RndOffs2RbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RND_OFFS_3_G` reader - Rank Offset to Neighbor for set 3 green\n\n"]
pub type RndOffs3GR = crate::FieldReader;
#[doc = "Field `RND_OFFS_3_G` writer - Rank Offset to Neighbor for set 3 green\n\n"]
pub type RndOffs3GW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RND_OFFS_3_RB` reader - Rank Offset to Neighbor for set 3 red/blue"]
pub type RndOffs3RbR = crate::FieldReader;
#[doc = "Field `RND_OFFS_3_RB` writer - Rank Offset to Neighbor for set 3 red/blue"]
pub type RndOffs3RbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Rank Offset to Neighbor for set 1 green"]
    #[inline(always)]
    pub fn rnd_offs_1_g(&self) -> RndOffs1GR {
        RndOffs1GR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Rank Offset to Neighbor for set 1 red/blue"]
    #[inline(always)]
    pub fn rnd_offs_1_rb(&self) -> RndOffs1RbR {
        RndOffs1RbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Rank Offset to Neighbor for set 2 green"]
    #[inline(always)]
    pub fn rnd_offs_2_g(&self) -> RndOffs2GR {
        RndOffs2GR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Rank Offset to Neighbor for set 2 red/blue"]
    #[inline(always)]
    pub fn rnd_offs_2_rb(&self) -> RndOffs2RbR {
        RndOffs2RbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Rank Offset to Neighbor for set 3 green\n\n"]
    #[inline(always)]
    pub fn rnd_offs_3_g(&self) -> RndOffs3GR {
        RndOffs3GR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Rank Offset to Neighbor for set 3 red/blue"]
    #[inline(always)]
    pub fn rnd_offs_3_rb(&self) -> RndOffs3RbR {
        RndOffs3RbR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Rank Offset to Neighbor for set 1 green"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_offs_1_g(&mut self) -> RndOffs1GW<DpccRndOffsSpec> {
        RndOffs1GW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Rank Offset to Neighbor for set 1 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_offs_1_rb(&mut self) -> RndOffs1RbW<DpccRndOffsSpec> {
        RndOffs1RbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Rank Offset to Neighbor for set 2 green"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_offs_2_g(&mut self) -> RndOffs2GW<DpccRndOffsSpec> {
        RndOffs2GW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Rank Offset to Neighbor for set 2 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_offs_2_rb(&mut self) -> RndOffs2RbW<DpccRndOffsSpec> {
        RndOffs2RbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Rank Offset to Neighbor for set 3 green\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_offs_3_g(&mut self) -> RndOffs3GW<DpccRndOffsSpec> {
        RndOffs3GW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Rank Offset to Neighbor for set 3 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_offs_3_rb(&mut self) -> RndOffs3RbW<DpccRndOffsSpec> {
        RndOffs3RbW::new(self, 10)
    }
}
#[doc = "Differential Rank Offsets for Rank Neighbor Difference\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rnd_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rnd_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRndOffsSpec;
impl crate::RegisterSpec for DpccRndOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_rnd_offs::R`](R) reader structure"]
impl crate::Readable for DpccRndOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_rnd_offs::W`](W) writer structure"]
impl crate::Writable for DpccRndOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RND_OFFS to value 0"]
impl crate::Resettable for DpccRndOffsSpec {
    const RESET_VALUE: u32 = 0;
}
