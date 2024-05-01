#[doc = "Register `CMD_CFG` reader"]
pub type R = crate::R<CmdCfgSpec>;
#[doc = "Register `CMD_CFG` writer"]
pub type W = crate::W<CmdCfgSpec>;
#[doc = "Field `REAL_ADDR` reader - real address\n\nreal address"]
pub type RealAddrR = crate::FieldReader<u16>;
#[doc = "Field `REAL_ADDR` writer - real address\n\nreal address"]
pub type RealAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CMD_MASK` reader - command mask of bit31~bit16\n\ncommand mask of bit31~bit16"]
pub type CmdMaskR = crate::FieldReader<u16>;
#[doc = "Field `CMD_MASK` writer - command mask of bit31~bit16\n\ncommand mask of bit31~bit16"]
pub type CmdMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - real address\n\nreal address"]
    #[inline(always)]
    pub fn real_addr(&self) -> RealAddrR {
        RealAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - command mask of bit31~bit16\n\ncommand mask of bit31~bit16"]
    #[inline(always)]
    pub fn cmd_mask(&self) -> CmdMaskR {
        CmdMaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - real address\n\nreal address"]
    #[inline(always)]
    #[must_use]
    pub fn real_addr(&mut self) -> RealAddrW<CmdCfgSpec> {
        RealAddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - command mask of bit31~bit16\n\ncommand mask of bit31~bit16"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_mask(&mut self) -> CmdMaskW<CmdCfgSpec> {
        CmdMaskW::new(self, 16)
    }
}
#[doc = "command config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdCfgSpec;
impl crate::RegisterSpec for CmdCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_cfg::R`](R) reader structure"]
impl crate::Readable for CmdCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_cfg::W`](W) writer structure"]
impl crate::Writable for CmdCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_CFG to value 0xf000_f000"]
impl crate::Resettable for CmdCfgSpec {
    const RESET_VALUE: u32 = 0xf000_f000;
}
