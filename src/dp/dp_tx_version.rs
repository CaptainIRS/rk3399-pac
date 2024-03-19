#[doc = "Register `DP_TX_VERSION` reader"]
pub type R = crate::R<DpTxVersionSpec>;
#[doc = "Register `DP_TX_VERSION` writer"]
pub type W = crate::W<DpTxVersionSpec>;
#[doc = "Field `DP_TX_VERSION` reader - &lt;7:5> process \n\n011 (Global Foundry 28nm) \n\n&lt;4:3>: Version \n\n00: Rev A \n\n&lt;2:0>: Minor revision \n\n000: rev .1 \n\nNotion: It is effective when PAD_DVDD is \n\nsupplied"]
pub type DpTxVersionR = crate::FieldReader<u32>;
#[doc = "Field `DP_TX_VERSION` writer - &lt;7:5> process \n\n011 (Global Foundry 28nm) \n\n&lt;4:3>: Version \n\n00: Rev A \n\n&lt;2:0>: Minor revision \n\n000: rev .1 \n\nNotion: It is effective when PAD_DVDD is \n\nsupplied"]
pub type DpTxVersionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - &lt;7:5> process \n\n011 (Global Foundry 28nm) \n\n&lt;4:3>: Version \n\n00: Rev A \n\n&lt;2:0>: Minor revision \n\n000: rev .1 \n\nNotion: It is effective when PAD_DVDD is \n\nsupplied"]
    #[inline(always)]
    pub fn dp_tx_version(&self) -> DpTxVersionR {
        DpTxVersionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - &lt;7:5> process \n\n011 (Global Foundry 28nm) \n\n&lt;4:3>: Version \n\n00: Rev A \n\n&lt;2:0>: Minor revision \n\n000: rev .1 \n\nNotion: It is effective when PAD_DVDD is \n\nsupplied"]
    #[inline(always)]
    #[must_use]
    pub fn dp_tx_version(&mut self) -> DpTxVersionW<DpTxVersionSpec> {
        DpTxVersionW::new(self, 0)
    }
}
#[doc = "DP_TX version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_tx_version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_tx_version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpTxVersionSpec;
impl crate::RegisterSpec for DpTxVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_tx_version::R`](R) reader structure"]
impl crate::Readable for DpTxVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_tx_version::W`](W) writer structure"]
impl crate::Writable for DpTxVersionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_TX_VERSION to value 0x60"]
impl crate::Resettable for DpTxVersionSpec {
    const RESET_VALUE: u32 = 0x60;
}
